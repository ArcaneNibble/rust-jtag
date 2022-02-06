use bitvec::prelude::*;
use jtag::*;

use std::io::{Read, Write};
use std::net::{TcpListener};

fn main() {
    println!("Hello world!");

    let listener = TcpListener::bind("0.0.0.0:2542").unwrap();
    let (mut sock, addr) = listener.accept().unwrap();
    println!("connected to {:?}", addr);
    sock.set_nodelay(true).unwrap();
    
    let mut current_state = JTAGState::TestLogicReset;

    loop {
        let mut cmdbuf = [0u8; 16];
        let mut tmsbuf = [0u8; 512];
        let mut tdibuf = [0u8; 512];
        let tdobuf = [0u8; 512];

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
                // this seems busted no matter what?
                sock.write(b"xvcServer_v1.0:512\n").unwrap();
            },
            [b's', b'h'] => {
                sock.read(&mut cmdbuf[..6]).unwrap();
                if cmdbuf[..6] != [b's', b'h', b'i', b'f', b't', b':'] {
                    println!("unknown command! {:x?}", cmdbuf);
                    continue;
                }
                sock.read(&mut cmdbuf[..4]).unwrap();
                let num_bits = (cmdbuf[0] as usize) |
                                ((cmdbuf[1] as usize) << 8) |
                                ((cmdbuf[2] as usize) << 16) |
                                ((cmdbuf[3] as usize) << 24);
                let num_bytes = (num_bits + 7) / 8;
                println!("shift {} bits ({} bytes)", num_bits, num_bytes);
                if num_bytes > 512 {
                    println!("too long!");
                    continue;
                }
                sock.read(&mut tmsbuf[..num_bytes]).unwrap();
                sock.read(&mut tdibuf[..num_bytes]).unwrap();

                // println!("{:x?} {:x?}", &tmsbuf[..num_bytes], &tdibuf[..num_bytes]);

                let tms_vec = &tmsbuf.view_bits::<Lsb0>()[..num_bits];
                let tdi_vec = &tmsbuf.view_bits::<Lsb0>()[..num_bits];

                // println!("{:x?} {:x?}", tms_vec, tdi_vec);

                for i in 0..num_bits {
                    let tms = tms_vec[i];
                    let _tdi = tdi_vec[i];
                    current_state = current_state.transition(tms);
                    println!("--> {:?}", current_state);
                }

                sock.write(&tdobuf[..num_bytes]).unwrap();
            },
            _ => {
                println!("unknown command! peeked {:x?}", cmdbuf);
            }
        }
    }
}
