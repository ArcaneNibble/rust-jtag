use crate::*;

use bitvec::prelude::*;
use ftdi_mpsse::*;

pub struct FTDIJTAG {
    jtag_state: JTAGAdapterState,
    chunkshift_state: ChunkShifterJTAGAdapterState,
    ftdi: ftdi::Device,
}
impl AsMut<JTAGAdapterState> for FTDIJTAG {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl AsMut<ChunkShifterJTAGAdapterState> for FTDIJTAG {
    fn as_mut(&mut self) -> &mut ChunkShifterJTAGAdapterState {
        &mut self.chunkshift_state
    }
}
impl FTDIJTAG {
    pub fn new() -> Self {
        println!("new");

        let mut device = ftdi::find_by_vid_pid(0x0403, 0x8028).open().unwrap();

        let mpsse = ftdi_mpsse::MpsseSettings {
            reset: true,
            in_transfer_size: 64 * 1024,
            read_timeout: std::time::Duration::from_secs(1),
            write_timeout: std::time::Duration::from_secs(1),
            latency_timer: std::time::Duration::from_millis(10),
            mask: 0b1011,
            clock_frequency: Some(1_000_000),
        };

        device.init(&mpsse).unwrap();

        mpsse! {
            const (INIT_DATA, INIT_LEN) = {
                set_gpio_lower(0b0000, 0b1011);
            };
        }

        assert_eq!(INIT_LEN, 0);
        device.send(&INIT_DATA).unwrap();

        Self {
            jtag_state: JTAGAdapterState::new(),
            chunkshift_state: ChunkShifterJTAGAdapterState::new(),
            ftdi: device,
        }
    }
}

impl ChunkShifterJTAGAdapter for FTDIJTAG {
    fn delay_ns(&mut self, ns: u64) -> u64 {
        std::thread::sleep(std::time::Duration::from_nanos(ns));
        ns
    }
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64 {
        println!("ignoring clock speed {clk_hz} hz");
        clk_hz
    }

    fn shift_tms_chunk(&mut self, tms_chunk: &BitSlice) {
        println!("shift tms {tms_chunk:b}");

        let mut bytes = Vec::new();

        for subchunk in tms_chunk.chunks(7) {
            bytes.push(ClockTMSOut::NegEdge as u8); // tms out on -ve
            bytes.push((subchunk.len() - 1) as u8);
            let mut thisbyte = 0u8;
            thisbyte.view_bits_mut::<Lsb0>()[..subchunk.len()].clone_from_bitslice(subchunk);
            bytes.push(thisbyte);
        }

        println!("the resulting buffer is {bytes:x?}");

        self.ftdi.send(&bytes).unwrap();
    }
    fn shift_tdi_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) {
        println!("shift tdi {tdi_chunk:b} tms? {tms_exit}");

        // this is the main portion that will be sent as bytes
        let tdi_chunk_ = if tms_exit {
            assert!(tdi_chunk.len() > 1); // XXX error reporting?
            &tdi_chunk[..(tdi_chunk.len() - 1)]
        } else {
            tdi_chunk
        };

        let mut mpsse_bytes = Vec::new();

        for subchunk in tdi_chunk_.chunks(65536 * 8) {
            // bytes portion first
            let chunk_bytes = subchunk.len() / 8; // deliberate truncate

            mpsse_bytes.push(ClockDataOut::LsbNeg as u8); // tdi out on -ve
            mpsse_bytes.push((chunk_bytes - 1) as u8);
            mpsse_bytes.push(((chunk_bytes - 1) >> 8) as u8);

            let cur_mpsse_len = mpsse_bytes.len();
            mpsse_bytes.resize(cur_mpsse_len + chunk_bytes, 0u8);
            mpsse_bytes[cur_mpsse_len..cur_mpsse_len + chunk_bytes]
                .view_bits_mut::<Lsb0>()
                .clone_from_bitslice(&subchunk[..chunk_bytes * 8]);

            // bits portion
            let chunk_bits = subchunk.len() % 8;
            if chunk_bits != 0 {
                mpsse_bytes.push(ClockBitsOut::LsbNeg as u8); // tdi out on -ve
                mpsse_bytes.push((chunk_bits - 1) as u8);
                let mut thisbyte = 0u8;
                thisbyte.view_bits_mut::<Lsb0>()[..chunk_bits]
                    .clone_from_bitslice(&subchunk[chunk_bytes * 8..]);
                mpsse_bytes.push(thisbyte);
            }
        }

        if tms_exit {
            // handle TMS
            mpsse_bytes.push(ClockTMSOut::NegEdge as u8); // tms out on -ve
            mpsse_bytes.push(0);
            if tdi_chunk[tdi_chunk.len() - 1] {
                mpsse_bytes.push(0b10000001);
            } else {
                mpsse_bytes.push(0b00000001);
            }
        }

        println!("the resulting buffer is {mpsse_bytes:x?}");
        self.ftdi.send(&mpsse_bytes).unwrap();
    }
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) -> BitVec {
        println!("shift tditdo {tdi_chunk:b} tms? {tms_exit}");

        // this is the main portion that will be sent as bytes
        let tdi_chunk_ = if tms_exit {
            assert!(tdi_chunk.len() > 1); // XXX error reporting?
            &tdi_chunk[..(tdi_chunk.len() - 1)]
        } else {
            tdi_chunk
        };

        let mut mpsse_bytes = Vec::new();
        let mut rxbytes_bufsz = 0;
        let mut rxbytes_bytes = 0;
        let mut rxbytes_bits = 0;

        for subchunk in tdi_chunk_.chunks(65536 * 8) {
            // bytes portion first
            let chunk_bytes = subchunk.len() / 8; // deliberate truncate

            mpsse_bytes.push(ClockData::LsbPosIn as u8); // tdi out on -ve, in on +ve
            mpsse_bytes.push((chunk_bytes - 1) as u8);
            mpsse_bytes.push(((chunk_bytes - 1) >> 8) as u8);

            let cur_mpsse_len = mpsse_bytes.len();
            mpsse_bytes.resize(cur_mpsse_len + chunk_bytes, 0u8);
            mpsse_bytes[cur_mpsse_len..cur_mpsse_len + chunk_bytes]
                .view_bits_mut::<Lsb0>()
                .clone_from_bitslice(&subchunk[..chunk_bytes * 8]);

            rxbytes_bufsz += chunk_bytes;
            rxbytes_bytes += chunk_bytes;

            // bits portion
            let chunk_bits = subchunk.len() % 8;
            if chunk_bits != 0 {
                mpsse_bytes.push(ClockBits::LsbPosIn as u8); // tdi out on -ve, in on +ve
                mpsse_bytes.push((chunk_bits - 1) as u8);
                let mut thisbyte = 0u8;
                thisbyte.view_bits_mut::<Lsb0>()[..chunk_bits]
                    .clone_from_bitslice(&subchunk[chunk_bytes * 8..]);
                mpsse_bytes.push(thisbyte);

                rxbytes_bufsz += 1;
                rxbytes_bits += chunk_bits;
            }
        }

        if tms_exit {
            // handle TMS
            mpsse_bytes.push(ClockTMS::NegTMSPosTDO as u8); // tms out on -ve, in on +ve
            mpsse_bytes.push(0);
            if tdi_chunk[tdi_chunk.len() - 1] {
                mpsse_bytes.push(0b10000001);
            } else {
                mpsse_bytes.push(0b00000001);
            }
            rxbytes_bufsz += 1;
        }

        mpsse_bytes.push(MpsseCmd::SendImmediate as u8);

        println!("the resulting buffer is {mpsse_bytes:x?} rx {rxbytes_bufsz}");
        self.ftdi.send(&mpsse_bytes).unwrap();

        let mut rxbytebuf = vec![0; rxbytes_bufsz];
        self.ftdi.recv(&mut rxbytebuf).unwrap();
        println!("got back {rxbytebuf:x?}");

        let mut ret: BitVec = BitVec::with_capacity(tdi_chunk.len());
        ret.extend_from_bitslice(rxbytebuf[..rxbytes_bytes].view_bits::<Lsb0>());
        if rxbytes_bits > 0 {
            ret.extend_from_bitslice(
                &rxbytebuf[rxbytes_bytes].view_bits::<Lsb0>()[8 - rxbytes_bits..],
            );
        }
        if tms_exit {
            ret.push((rxbytebuf[rxbytebuf.len() - 1] & 0x80) != 0);
        }

        ret
    }
}
