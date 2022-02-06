use crate::*;

struct TestNativeJTAGAdapter<'a> {
    jtag_state: JTAGAdapterState,
    test_actions: &'a [JTAGAction],
}
impl<'a> AsMut<JTAGAdapterState> for TestNativeJTAGAdapter<'a> {
    fn as_mut(&mut self) -> &mut JTAGAdapterState {
        &mut self.jtag_state
    }
}
impl<'a> TestNativeJTAGAdapter<'a> {
    fn new(test_actions: &'a [JTAGAction]) -> Self {
        Self {
            jtag_state: JTAGAdapterState::new(),
            test_actions,
        }
    }
}

impl<'a> JTAGAdapter for TestNativeJTAGAdapter<'a> {
    fn execute_actions(&mut self, actions: &[JTAGAction]) -> Vec<JTAGOutput> {
        assert_eq!(actions, self.test_actions);
        vec![JTAGOutput::NoData; actions.len()]
    }
}

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
fn compile_test_trait_obj_safe() {
    let _: &dyn JTAGAdapter;
}

#[test]
fn test_native_adapter_fns() {
    let mut testadapter = TestNativeJTAGAdapter::new(&[]);

    testadapter.test_actions = &[JTAGAction::ResetToTLR];
    testadapter.reset_to_tlr();
    testadapter.flush();
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
