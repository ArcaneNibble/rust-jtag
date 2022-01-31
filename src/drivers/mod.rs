use crate::*;

use ftdi_mpsse::{mpsse, MpsseCmdExecutor};

pub struct CrabbyTTYPreAlphaJTAG {
    jtag_state: JTAGAdapterState,
    chunkshift_state: ChunkShifterJTAGAdapterState,
    bitbang_state: BitbangJTAGAdapterState,
    usb: rusb::DeviceHandle<rusb::GlobalContext>,
}
impl AsMut<JTAGAdapterState> for CrabbyTTYPreAlphaJTAG {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl AsMut<ChunkShifterJTAGAdapterState> for CrabbyTTYPreAlphaJTAG {
    fn as_mut(&mut self) -> &mut ChunkShifterJTAGAdapterState {
        &mut self.chunkshift_state
    }
}
impl AsMut<BitbangJTAGAdapterState> for CrabbyTTYPreAlphaJTAG {
    fn as_mut(&mut self) -> &mut BitbangJTAGAdapterState {
        &mut self.bitbang_state
    }
}
impl CrabbyTTYPreAlphaJTAG {
    pub fn new() -> Self {
        println!("new");

        let device = rusb::open_device_with_vid_pid(0xf055, 0x0000).unwrap();
        device
            .write_control(0x40, 1, 0, 0, &[], std::time::Duration::from_secs(1))
            .unwrap();

        Self {
            jtag_state: JTAGAdapterState::new(),
            chunkshift_state: ChunkShifterJTAGAdapterState::new(),
            bitbang_state: BitbangJTAGAdapterState::new(),
            usb: device,
        }
    }
}

impl BitbangJTAGAdapter for CrabbyTTYPreAlphaJTAG {
    fn set_clk_speed(&mut self, clk_hz: u64) {
        println!("ignoring clock speed {clk_hz} hz");
    }

    fn shift_one_bit(&mut self, tms: bool, tdi: bool) -> bool {
        let mut reqbyte = 0u8;

        if tdi {
            reqbyte |= 0b01;
        }
        if tms {
            reqbyte |= 0b10;
        }

        let resbyte = &mut [0u8];
        let usbret = self.usb.read_control(
            0xC0,
            3,
            reqbyte as u16,
            0,
            resbyte,
            std::time::Duration::from_secs(1),
        );
        assert_eq!(usbret.unwrap(), 1);

        println!("tms {tms} tdi {tdi} --> {resbyte:?}");

        resbyte[0] & 1 != 0
    }
}

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
    fn delay_ns(&mut self, ns: u64) {
        std::thread::sleep(std::time::Duration::from_nanos(ns))
    }
    fn set_clk_speed(&mut self, clk_hz: u64) {
        println!("ignoring clock speed {clk_hz} hz");
    }

    fn shift_tms_chunk(&mut self, tms_chunk: &BitSlice) {
        println!("shift tms {tms_chunk:?}");

        let mut bytes = Vec::new();
        let mut bits_remaining = tms_chunk.len();
        let mut inbitsi = 0;

        while bits_remaining > 0 {
            let bits = if bits_remaining > 7 {
                7
            } else {
                bits_remaining
            };
            let mut thisbyte = 0u8;
            for i in 0..bits {
                if tms_chunk[inbitsi + i] {
                    thisbyte |= 1 << i;
                }
            }

            bytes.push(0b01001011); // tms out on -ve
            bytes.push((bits - 1) as u8);
            bytes.push(thisbyte);

            bits_remaining -= bits;
            inbitsi += bits;
        }

        println!("the resulting buffer is {bytes:?}");

        self.ftdi.send(&bytes).unwrap();
    }
    fn shift_tdi_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) {
        println!("shift tdi {tdi_chunk:?} tms? {tms_exit}");

        // super fixme
        self.shift_tditdo_chunk(tdi_chunk, tms_exit);
    }
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) -> BitVec {
        println!("shift tditdo {tdi_chunk:?} tms? {tms_exit}");

        assert!(tdi_chunk.len() > 1); // XXX

        let mut bytes = Vec::new();
        let mut rxbytes = 0;
        let mut bits_remaining = tdi_chunk.len() - 1; // need special TMS
        let mut inbitsi = 0;

        while bits_remaining > 0 {
            // fixme this is super inefficient
            let bits = if bits_remaining > 8 {
                8
            } else {
                bits_remaining
            };
            let mut thisbyte = 0u8;
            for i in 0..bits {
                if tdi_chunk[inbitsi + i] {
                    thisbyte |= 1 << i;
                }
            }

            bytes.push(0b00111011); // tdi out on -ve, in on +ve
            bytes.push((bits - 1) as u8);
            bytes.push(thisbyte);

            rxbytes += 1;
            bits_remaining -= bits;
            inbitsi += bits;
        }

        // handle TMS
        bytes.push(0b01101011); // tms out on -ve, in on +ve
        bytes.push(0);
        if tms_exit {
            if tdi_chunk[tdi_chunk.len() - 1] {
                bytes.push(0b10000001);
            } else {
                bytes.push(0b00000001);
            }
        } else {
            if tdi_chunk[tdi_chunk.len() - 1] {
                bytes.push(0b10000000);
            } else {
                bytes.push(0b00000000);
            }
        }
        rxbytes += 1;

        // wtf?
        bytes.push(0x87);

        println!("the resulting buffer is {bytes:?} rx {rxbytes}");
        self.ftdi.send(&bytes).unwrap();

        let mut rxbytebuf = vec![0; rxbytes];
        self.ftdi.recv(&mut rxbytebuf).unwrap();
        println!("got back {rxbytebuf:?}");

        // fixme fixme fixme
        let mut ret = Vec::new();
        let mut bits_remaining = tdi_chunk.len() - 1;
        let mut rxbytebuf_i = 0;
        while bits_remaining > 0 {
            let bits = if bits_remaining > 8 {
                8
            } else {
                bits_remaining
            };

            for i in (8 - bits)..8 {
                ret.push((rxbytebuf[rxbytebuf_i] & (1 << i)) != 0);
            }

            bits_remaining -= bits;
            rxbytebuf_i += 1;
        }
        assert_eq!(rxbytebuf_i, rxbytebuf.len() - 1);
        ret.push((rxbytebuf[rxbytebuf.len() - 1] & 0x80) != 0);
        assert_eq!(ret.len(), tdi_chunk.len());

        ret.iter().collect()
    }
}
