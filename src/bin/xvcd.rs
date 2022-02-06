use bitvec::prelude::*;
use jtag::*;

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Hello world!");

    let mut adapter = jtag::drivers::FTDIJTAG::new();

    let listener = TcpListener::bind("0.0.0.0:2542").unwrap();
    let (mut sock, addr) = listener.accept().unwrap();
    println!("connected to {:?}", addr);
    sock.set_nodelay(true).unwrap();

    let mut current_state = JTAGState::TestLogicReset;

    loop {
        let mut cmdbuf = [0u8; 16];
        let mut tmsbuf = [0u8; 512];
        let mut tdibuf = [0u8; 512];
        let mut tdobuf;

        let peeklen = sock.peek(&mut cmdbuf[..2]).unwrap();
        if peeklen != 2 {
            println!("bad read");
            break;
        }
        match cmdbuf[..2] {
            [b'g', b'e'] => {
                sock.read(&mut cmdbuf[..8]).unwrap();
                if cmdbuf[..8] != [b'g', b'e', b't', b'i', b'n', b'f', b'o', b':'] {
                    println!("unknown command! {:x?}", cmdbuf);
                    continue;
                }
                sock.write(b"xvcServer_v1.0:5120\n").unwrap();
            }
            [b's', b'h'] => {
                sock.read(&mut cmdbuf[..6]).unwrap();
                if cmdbuf[..6] != [b's', b'h', b'i', b'f', b't', b':'] {
                    println!("unknown command! {:x?}", cmdbuf);
                    continue;
                }
                sock.read(&mut cmdbuf[..4]).unwrap();
                let num_bits = (cmdbuf[0] as usize)
                    | ((cmdbuf[1] as usize) << 8)
                    | ((cmdbuf[2] as usize) << 16)
                    | ((cmdbuf[3] as usize) << 24);
                let num_bytes = (num_bits + 7) / 8;
                println!("shift {} bits ({} bytes)", num_bits, num_bytes);
                if num_bytes > 512 {
                    println!("too long!");
                    continue;
                }
                sock.read(&mut tmsbuf[..num_bytes]).unwrap();
                sock.read(&mut tdibuf[..num_bytes]).unwrap();

                tdobuf = tdibuf.clone();

                println!("{:x?} {:x?}", &tmsbuf[..num_bytes], &tdibuf[..num_bytes]);

                let tms_vec = &tmsbuf.view_bits::<Lsb0>()[..num_bits];
                let tdi_vec = &tdibuf.view_bits::<Lsb0>()[..num_bits];
                let tdo_vec = &mut tdobuf.view_bits_mut::<Lsb0>()[..num_bits];

                // println!("{:x?} {:x?}", tms_vec, tdi_vec);

                let mut q = Vec::new();
                let mut q2 = Vec::new();
                let mut accum_states = Vec::new();
                let mut is_shifting = false;
                let mut shifting_bits = 0;
                let mut shift_start_idx = 0;

                let mut i = 0;
                while i < num_bits {
                    if i + 5 <= num_bits && tms_vec[i..i + 5] == bits![1; 5] {
                        q.push(JTAGAction::ResetToTLR);
                        q2.push((0, 0));
                        current_state = JTAGState::TestLogicReset;
                        let tdi = &tdi_vec[i..i + 5];
                        if tdi != bits![0; 5] && tdi != bits![1; 5] {
                            println!("Warn: unknown TDI data (reset TLR) {:?}", tdi);
                        }

                        i += 5;
                        continue;
                    }

                    let tms = tms_vec[i];
                    // let _tdi = tdi_vec[i];
                    if current_state == JTAGState::ShiftDR || current_state == JTAGState::ShiftIR {
                        is_shifting = true;
                        // println!("shift a bit");
                        shifting_bits += 1;
                        current_state = current_state.transition(tms);
                        if current_state != JTAGState::ShiftDR
                            && current_state != JTAGState::ShiftIR
                        {
                            println!("shift exited! {} bits", shifting_bits);
                            q.push(JTAGAction::ShiftBits {
                                bits_tdi: tdi_vec[shift_start_idx..i + 1].iter().collect(),
                                capture: true,
                                tms_exit: true,
                            });
                            q2.push((shift_start_idx, i + 1));
                            is_shifting = false;
                        }
                    } else {
                        current_state = current_state.transition(tms);
                        println!("--> {:?}", current_state);
                        accum_states.push(current_state);

                        if current_state == JTAGState::ShiftDR
                            || current_state == JTAGState::ShiftIR
                        {
                            println!("got to a shifting state!");
                            is_shifting = true;
                            q.push(JTAGAction::GoViaStates(accum_states.split_off(0)));
                            q2.push((0, 0));
                            shift_start_idx = i + 1;
                        }
                    }

                    i += 1;
                }

                if is_shifting && shifting_bits > 0 {
                    println!("shift didn't exit! {} bits", shifting_bits);
                    q.push(JTAGAction::ShiftBits {
                        bits_tdi: tdi_vec[shift_start_idx..i].iter().collect(),
                        capture: true,
                        tms_exit: false,
                    });
                    q2.push((shift_start_idx, i));
                }
                if accum_states.len() > 0 {
                    q.push(JTAGAction::GoViaStates(accum_states));
                    q2.push((0, 0));
                }
                println!("q {:?}", q);
                println!("q2 {:?}", q2);
                assert_eq!(q.len(), q2.len());

                let result = adapter.execute_actions(&q);
                // println!("out {:?}", result);
                assert_eq!(q2.len(), result.len());

                for i in 0..result.len() {
                    if let JTAGOutput::CapturedBits(bv) = &result[i] {
                        let (idxstart, idxend) = q2[i];
                        println!("copying to ({}, {})", idxstart, idxend);
                        tdo_vec[idxstart..idxend].clone_from_bitslice(bv);
                    }
                }

                // println!("{:x?}", tdo_vec);

                sock.write(&tdobuf[..num_bytes]).unwrap();
            }
            _ => {
                println!("unknown command! peeked {:x?}", cmdbuf);
            }
        }
    }
}
