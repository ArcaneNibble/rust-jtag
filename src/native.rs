use crate::*;

use bitvec::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[non_exhaustive]
/// Represents all possible actions that can be performed on a JTAG adapter
pub enum JTAGAction {
    /// Wait for a given number of nanoseconds
    DelayNS(u64),
    /// Set the JTAG clock speed in Hz
    SetClkSpeed(u64),

    /// Send 5 TMS=1 transitions, bringing the TAP to Test-Logic-Reset starting
    /// from any state
    ResetToTLR,
    /// Transition the TAP through the specified states via the shortest
    /// possible path
    GoViaStates(Vec<JTAGState>),

    /// Shift [bits_tdi][Self::ShiftBits::bits_tdi] into the TDI pin.
    /// If [capture][Self::ShiftBits::capture] is `true`, TDO data will be
    /// captured. If [tms_exit][Self::ShiftBits::tms_exit] is `true`,
    /// the final bit will be shifted with TMS=1 (which would cause a
    /// transition from Shift-IR/DR to Exit1-IR/DR). Otherwise, the final bit
    /// will be shifted with TMS=0. All other bits will be shifted with TMS=0.
    ShiftBits {
        bits_tdi: BitVec,
        capture: bool,
        tms_exit: bool,
    },

    /// Take the TAP from its current state to Shift-IR, then shift in the
    /// data from [ir][Self::ShiftIR::ir] while capturing data if
    /// [capture][Self::ShiftIR::capture] is `true`.
    /// If [pause][Self::ShiftIR::pause] is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-IR.
    ShiftIR {
        ir: BitVec,
        capture: bool,
        pause: bool,
    },
    /// Take the TAP from its current state to Shift-DR, then shift in the
    /// data from [dr][Self::ShiftDR::dr] while capturing data if
    /// [capture][Self::ShiftDR::capture] is `true`.
    /// If [pause][Self::ShiftDR::pause] is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-DR.
    ShiftDR {
        dr: BitVec,
        capture: bool,
        pause: bool,
    },

    /// If the current IR value is not equal to this value, take the TAP to
    /// Shift-IR, shift in this value, and then take the TAP to Run-Test/Idle.
    /// If the current IR value is equal, do nothing.
    SetIR(BitVec),
    /// Set the current IR to this value if it is not already, and then
    /// take the TAP to Shift-DR. Capture the data, and then take the TAP to
    /// Run-Test/Idle.
    ReadReg { ir: BitVec, drlen: usize },
    /// Set the current IR to [ir][Self::WriteReg::ir] if it is not already,
    /// and then take the TAP to Shift-DR. Shift in the data from
    /// [dr][Self::WriteReg::dr], and then take the TAP to Run-Test/Idle.
    WriteReg { ir: BitVec, dr: BitVec },
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[non_exhaustive]
/// Represents possible results from performing JTAG actions
pub enum JTAGOutput {
    /// No data was stored for the corresponding JTAG action
    NoData,
    /// Captured TDO data for the corresponding JTAG action
    CapturedBits(BitVec),
    /// Actual number of nanoseconds delayed by a
    /// [DelayNS][JTAGAction::DelayNS] command
    ActualDelay(u64),
    /// Actual clock speed in Hz set by a
    /// [SetClkSpeed][JTAGAction::SetClkSpeed] command
    ActualClkSpeed(u64),
}

#[derive(Clone, Debug)]
/// State required to be stored by a [JTAGAdapter]
pub struct JTAGAdapterState {
    queued_actions: Vec<JTAGAction>,
}
impl JTAGAdapterState {
    pub fn new() -> Self {
        Self {
            queued_actions: Vec::new(),
        }
    }
}

/// Trait representing a generic JTAG adapter
// FIXME: Properly document the buffering story
pub trait JTAGAdapter: AsMut<JTAGAdapterState> {
    /// Execute all of the passed-in JTAG actions immediately without buffering
    /// and return their results
    fn execute_actions(&mut self, actions: &[JTAGAction]) -> Vec<JTAGOutput>;

    /// Execute all of the currently buffered JTAG actions and return their
    /// results
    fn flush(&mut self) -> Vec<JTAGOutput> {
        let state: &mut JTAGAdapterState = self.as_mut();
        let mut actions = state.queued_actions.split_off(0);
        self.execute_actions(&mut actions)
    }
    /// Add the given action to the buffered action queue and return
    /// immediately
    fn queue_action(&mut self, action: JTAGAction) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(action);
    }

    /// Send 5 TMS=1 transitions, bringing the TAP to Test-Logic-Reset starting
    /// from any state (buffered action, returns immediately)
    fn reset_to_tlr(&mut self) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ResetToTLR);
    }
    /// Transition the TAP through the specified states via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_via_states(&mut self, states: &[JTAGState]) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state
            .queued_actions
            .push(JTAGAction::GoViaStates(states.to_vec()));
    }

    /// Transition the TAP to the specified state via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_to_state(&mut self, state: JTAGState) {
        self.go_via_states(&[state]);
    }
    /// Transition the TAP to Test-Logic-Reset via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_tlr(&mut self) {
        self.go_to_state(JTAGState::TestLogicReset);
    }
    /// Transition the TAP to Run-Test/Idle via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_rti(&mut self) {
        self.go_to_state(JTAGState::RunTestIdle);
    }
    /// Transition the TAP to Select-DR-Scan via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_selectdr(&mut self) {
        self.go_to_state(JTAGState::SelectDR);
    }
    /// Transition the TAP to Capture-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_capturedr(&mut self) {
        self.go_to_state(JTAGState::CaptureDR);
    }
    /// Transition the TAP to Shift-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_shiftdr(&mut self) {
        self.go_to_state(JTAGState::ShiftDR);
    }
    /// Transition the TAP to Exit1-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_exit1dr(&mut self) {
        self.go_to_state(JTAGState::Exit1DR);
    }
    /// Transition the TAP to Pause-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_pausedr(&mut self) {
        self.go_to_state(JTAGState::PauseDR);
    }
    /// Transition the TAP to Exit2-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_exit2dr(&mut self) {
        self.go_to_state(JTAGState::Exit2DR);
    }
    /// Transition the TAP to Update-DR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_updatedr(&mut self) {
        self.go_to_state(JTAGState::UpdateDR);
    }
    /// Transition the TAP to Select-IR-Scan via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_selectir(&mut self) {
        self.go_to_state(JTAGState::SelectIR);
    }
    /// Transition the TAP to Capture-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_captureir(&mut self) {
        self.go_to_state(JTAGState::CaptureIR);
    }
    /// Transition the TAP to Shift-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_shiftir(&mut self) {
        self.go_to_state(JTAGState::ShiftIR);
    }
    /// Transition the TAP to Exit1-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_exit1ir(&mut self) {
        self.go_to_state(JTAGState::Exit1IR);
    }
    /// Transition the TAP to Pause-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_pauseir(&mut self) {
        self.go_to_state(JTAGState::PauseIR);
    }
    /// Transition the TAP to Exit2-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_exit2ir(&mut self) {
        self.go_to_state(JTAGState::Exit2IR);
    }
    /// Transition the TAP to Update-IR via the shortest
    /// possible path (buffered action, returns immediately)
    fn go_updateir(&mut self) {
        self.go_to_state(JTAGState::UpdateIR);
    }

    /// Shift bits into the TDI pin. TDO data will not be captured.
    /// If `tms_exit` is `true`, the final bit will be shifted with TMS=1
    /// (which would cause a transition from Shift-IR/DR to Exit1-IR/DR).
    /// Otherwise, the final bit will be shifted with TMS=0. All other bits
    /// will be shifted with TMS=0.
    ///
    /// This is a buffered action that returns immediately
    fn shift_bits_out(&mut self, bits: &BitSlice, tms_exit: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftBits {
            bits_tdi: bits.to_owned(),
            capture: false,
            tms_exit,
        });
    }
    /// Shift bits into the TDI pin. TDO data will be captured and returned.
    /// If `tms_exit` is `true`, the final bit will be shifted with TMS=1
    /// (which would cause a transition from Shift-IR/DR to Exit1-IR/DR).
    /// Otherwise, the final bit will be shifted with TMS=0. All other bits
    /// will be shifted with TMS=0.
    ///
    /// This is a blocking action that will additionally flush all pending
    /// buffered actions.
    fn shift_bits_inout(&mut self, bits: &BitSlice, tms_exit: bool) -> BitVec {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftBits {
            bits_tdi: bits.to_owned(),
            capture: true,
            tms_exit,
        });

        let mut ret = self.flush();
        let retlen = ret.len();
        if let JTAGOutput::CapturedBits(out) = &mut ret[retlen - 1] {
            out.split_off(0)
        } else {
            unreachable!()
        }
    }

    /// Take the TAP from its current state to Shift-IR, then shift in the
    /// data from `ir`. If `pause` is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-IR.
    ///
    /// This is a buffered action that returns immediately
    fn shift_ir_out(&mut self, ir: &BitSlice, pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftIR {
            ir: ir.to_owned(),
            capture: false,
            pause,
        });
    }
    /// Take the TAP from its current state to Shift-IR, then shift in the
    /// data from `ir`. The shifted-out data will be captured and returned.
    /// If `pause` is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-IR.
    ///
    /// This is a blocking action that will additionally flush all pending
    /// buffered actions.
    fn shift_ir_inout(&mut self, ir: &BitSlice, pause: bool) -> BitVec {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftIR {
            ir: ir.to_owned(),
            capture: true,
            pause,
        });

        let mut ret = self.flush();
        let retlen = ret.len();
        if let JTAGOutput::CapturedBits(out) = &mut ret[retlen - 1] {
            out.split_off(0)
        } else {
            unreachable!()
        }
    }
    /// Take the TAP from its current state to Shift-DR, then shift in the
    /// data from `dr`. If `pause` is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-DR.
    ///
    /// This is a buffered action that returns immediately
    fn shift_dr_out(&mut self, dr: &BitSlice, pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftDR {
            dr: dr.to_owned(),
            capture: true,
            pause,
        });
    }
    /// Take the TAP from its current state to Shift-DR, then shift in the
    /// data from `dr`. The shifted-out data will be captured and returned.
    /// If `pause` is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-DR.
    ///
    /// This is a blocking action that will additionally flush all pending
    /// buffered actions.
    fn shift_dr_inout(&mut self, dr: &BitSlice, pause: bool) -> BitVec {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftDR {
            dr: dr.to_owned(),
            capture: true,
            pause,
        });

        let mut ret = self.flush();
        let retlen = ret.len();
        if let JTAGOutput::CapturedBits(out) = &mut ret[retlen - 1] {
            out.split_off(0)
        } else {
            unreachable!()
        }
    }

    /// If the current IR value is not equal to this value, take the TAP to
    /// Shift-IR, shift in this value, and then take the TAP to Run-Test/Idle.
    /// If the current IR value is equal, do nothing.
    ///
    /// This is a buffered action that returns immediately
    fn set_ir(&mut self, ir: &BitSlice) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::SetIR(ir.to_owned()));
    }
    /// Set the current IR to this value if it is not already, and then
    /// take the TAP to Shift-DR. Capture the data, and then take the TAP to
    /// Run-Test/Idle.
    ///
    /// This is a blocking action that will additionally flush all pending
    /// buffered actions.
    fn read_reg(&mut self, ir: &BitSlice, drlen: usize) -> BitVec {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ReadReg {
            ir: ir.to_owned(),
            drlen,
        });

        let mut ret = self.flush();
        let retlen = ret.len();
        if let JTAGOutput::CapturedBits(out) = &mut ret[retlen - 1] {
            out.split_off(0)
        } else {
            unreachable!()
        }
    }
    /// Set the current IR to `ir` if it is not already,
    /// and then take the TAP to Shift-DR. Shift in the data from
    /// `dr`, and then take the TAP to Run-Test/Idle.
    ///
    /// This is a buffered action that returns immediately
    fn write_reg(&mut self, ir: &BitSlice, dr: &BitSlice) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::WriteReg {
            ir: ir.to_owned(),
            dr: dr.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use bitvec::prelude::*;

    #[test]
    fn compile_test_trait_obj_safe() {
        let _: &dyn JTAGAdapter;
    }

    struct TestNativeJTAGAdapter {
        jtag_state: JTAGAdapterState,
        test_actions: Vec<JTAGAction>,
    }
    impl AsMut<JTAGAdapterState> for TestNativeJTAGAdapter {
        fn as_mut(&mut self) -> &mut JTAGAdapterState {
            &mut self.jtag_state
        }
    }
    impl TestNativeJTAGAdapter {
        fn new(test_actions: &[JTAGAction]) -> Self {
            Self {
                jtag_state: JTAGAdapterState::new(),
                test_actions: test_actions.to_vec(),
            }
        }
    }

    impl JTAGAdapter for TestNativeJTAGAdapter {
        fn execute_actions(&mut self, actions: &[JTAGAction]) -> Vec<JTAGOutput> {
            assert_eq!(actions, self.test_actions);
            let mut ret = Vec::new();

            for action in actions {
                ret.push(match action {
                    JTAGAction::ShiftBits{bits_tdi, capture, ..} => {
                        if *capture {
                            JTAGOutput::CapturedBits(bits_tdi.clone())
                        } else {
                            JTAGOutput::NoData
                        }
                    },
                    _ => JTAGOutput::NoData,
                });
            };

            ret
        }
    }

    #[test]
    fn test_native_adapter_fns() {
        let mut testadapter = TestNativeJTAGAdapter::new(&[]);

        testadapter.test_actions = vec![JTAGAction::ResetToTLR];
        testadapter.reset_to_tlr();
        testadapter.flush();

        // test buffering
        testadapter.test_actions = vec![];
        testadapter.reset_to_tlr();
        testadapter.go_rti();
        testadapter.go_tlr();
        testadapter.shift_bits_out(bits![1, 0, 1, 0], false);
        testadapter.test_actions = vec![
            JTAGAction::ResetToTLR,
            JTAGAction::GoViaStates(vec![JTAGState::RunTestIdle]),
            JTAGAction::GoViaStates(vec![JTAGState::TestLogicReset]),
            JTAGAction::ShiftBits {
                bits_tdi: bitvec![1, 0, 1, 0],
                capture: false,
                tms_exit: false,
            },
            JTAGAction::ShiftBits {
                bits_tdi: bitvec![0, 1, 0, 1],
                capture: true,
                tms_exit: true,
            },
        ];
        testadapter.shift_bits_inout(bits![0, 1, 0, 1], true);
    }
}
