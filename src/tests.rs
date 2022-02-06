use crate::*;

use bitvec::prelude::*;

struct TestBitbangJTAGAdapter {
    jtag_state: JTAGAdapterState,
    chunkshift_state: ChunkShifterJTAGAdapterState,
    bitbang_state: BitbangJTAGAdapterState,
}
impl AsMut<JTAGAdapterState> for TestBitbangJTAGAdapter {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl AsMut<ChunkShifterJTAGAdapterState> for TestBitbangJTAGAdapter {
    fn as_mut(&mut self) -> &mut ChunkShifterJTAGAdapterState {
        &mut self.chunkshift_state
    }
}
impl AsMut<BitbangJTAGAdapterState> for TestBitbangJTAGAdapter {
    fn as_mut(&mut self) -> &mut BitbangJTAGAdapterState {
        &mut self.bitbang_state
    }
}
impl TestBitbangJTAGAdapter {
    fn new() -> Self {
        Self {
            jtag_state: JTAGAdapterState::new(),
            chunkshift_state: ChunkShifterJTAGAdapterState::new(),
            bitbang_state: BitbangJTAGAdapterState::new(),
        }
    }
}

impl BitbangJTAGAdapter for TestBitbangJTAGAdapter {
    fn set_clk_speed(&mut self, _clk_hz: u64) -> u64 {
        todo!()
    }

    fn shift_one_bit(&mut self, _tms: bool, _tdi: bool) -> bool {
        todo!()
    }
}

struct TestChunkJTAGAdapter {
    jtag_state: JTAGAdapterState,
    chunkshift_state: ChunkShifterJTAGAdapterState,
}
impl AsMut<JTAGAdapterState> for TestChunkJTAGAdapter {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl AsMut<ChunkShifterJTAGAdapterState> for TestChunkJTAGAdapter {
    fn as_mut(&mut self) -> &mut ChunkShifterJTAGAdapterState {
        &mut self.chunkshift_state
    }
}
impl TestChunkJTAGAdapter {
    fn new() -> Self {
        Self {
            jtag_state: JTAGAdapterState::new(),
            chunkshift_state: ChunkShifterJTAGAdapterState::new(),
        }
    }
}

impl ChunkShifterJTAGAdapter for TestChunkJTAGAdapter {
    fn delay_ns(&mut self, _ns: u64) -> u64 {
        todo!()
    }
    fn set_clk_speed(&mut self, _clk_hz: u64) -> u64 {
        todo!()
    }

    fn shift_tms_chunk(&mut self, _tms_chunk: &BitSlice) {
        todo!()
    }
    fn shift_tdi_chunk(&mut self, _tdi_chunk: &BitSlice, _tms_exit: bool) {
        todo!()
    }
    fn shift_tditdo_chunk(&mut self, _tdi_chunk: &BitSlice, _tms_exit: bool) -> BitVec {
        todo!()
    }
}

struct TestStateJTAGAdapter {
    jtag_state: JTAGAdapterState,
}
impl AsMut<JTAGAdapterState> for TestStateJTAGAdapter {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl TestStateJTAGAdapter {
    fn new() -> Self {
        Self {
            jtag_state: JTAGAdapterState::new(),
        }
    }
}

impl StateTrackingJTAGAdapter for TestStateJTAGAdapter {
    fn execute_stjtag_action(&mut self, _action: &JTAGAction) -> JTAGOutput {
        todo!()
    }
}

#[test]
fn test_bitbang_adapter() {
    let mut testadapter = TestBitbangJTAGAdapter::new();

    testadapter.reset_to_tlr();
}

#[test]
fn test_chunked_adapter() {
    let mut testadapter = TestChunkJTAGAdapter::new();

    testadapter.reset_to_tlr();
}

#[test]
fn test_state_tracking_adapter() {
    let mut testadapter = TestStateJTAGAdapter::new();

    testadapter.reset_to_tlr();
}
