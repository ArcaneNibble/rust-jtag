use crate::*;

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

        Self {
            jtag_state: JTAGAdapterState::new(),
            chunkshift_state: ChunkShifterJTAGAdapterState::new(),
        }
    }
}

impl ChunkShifterJTAGAdapter for FTDIJTAG {
    fn delay_ns(&mut self, _ns: u64) {
        todo!()
    }
    fn set_clk_speed(&mut self, _clk_hz: u64) {
        todo!()
    }

    fn shift_tms_chunk(&mut self, _tms_chunk: &[bool]) {
        todo!()
    }
    fn shift_tdi_chunk(&mut self, _tdi_chunk: &[bool], _tms_exit: bool) {
        todo!()
    }
    fn shift_tditdo_chunk(&mut self, _tdi_chunk: &[bool], _tms_exit: bool) -> Vec<bool> {
        todo!()
    }
}
