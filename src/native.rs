use crate::*;

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
    /// will be shifted with TMS=0.
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

pub trait JTAGAdapter: AsMut<JTAGAdapterState> {
    fn execute_actions(&mut self, actions: &[JTAGAction]) -> Vec<JTAGOutput>;

    fn flush(&mut self) -> Vec<JTAGOutput> {
        let state: &mut JTAGAdapterState = self.as_mut();
        let mut actions = state.queued_actions.split_off(0);
        self.execute_actions(&mut actions)
    }
    fn queue_action(&mut self, action: JTAGAction) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(action);
    }

    fn reset_to_tlr(&mut self) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ResetToTLR);
    }
    fn go_via_states(&mut self, states: &[JTAGState]) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state
            .queued_actions
            .push(JTAGAction::GoViaStates(states.to_vec()));
    }

    fn go_to_state(&mut self, state: JTAGState) {
        self.go_via_states(&[state]);
    }
    fn go_tlr(&mut self) {
        self.go_to_state(JTAGState::TestLogicReset);
    }
    fn go_rti(&mut self) {
        self.go_to_state(JTAGState::RunTestIdle);
    }
    fn go_selectdr(&mut self) {
        self.go_to_state(JTAGState::SelectDR);
    }
    fn go_capturedr(&mut self) {
        self.go_to_state(JTAGState::CaptureDR);
    }
    fn go_shiftdr(&mut self) {
        self.go_to_state(JTAGState::ShiftDR);
    }
    fn go_exit1dr(&mut self) {
        self.go_to_state(JTAGState::Exit1DR);
    }
    fn go_pausedr(&mut self) {
        self.go_to_state(JTAGState::PauseDR);
    }
    fn go_exit2dr(&mut self) {
        self.go_to_state(JTAGState::Exit2DR);
    }
    fn go_updatedr(&mut self) {
        self.go_to_state(JTAGState::UpdateDR);
    }
    fn go_selectir(&mut self) {
        self.go_to_state(JTAGState::SelectIR);
    }
    fn go_captureir(&mut self) {
        self.go_to_state(JTAGState::CaptureIR);
    }
    fn go_shiftir(&mut self) {
        self.go_to_state(JTAGState::ShiftIR);
    }
    fn go_exit1ir(&mut self) {
        self.go_to_state(JTAGState::Exit1IR);
    }
    fn go_pauseir(&mut self) {
        self.go_to_state(JTAGState::PauseIR);
    }
    fn go_exit2ir(&mut self) {
        self.go_to_state(JTAGState::Exit2IR);
    }
    fn go_updateir(&mut self) {
        self.go_to_state(JTAGState::UpdateIR);
    }

    fn shift_bits_out(&mut self, bits: &BitSlice, tms_exit: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftBits {
            bits_tdi: bits.to_owned(),
            capture: false,
            tms_exit,
        });
    }
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

    fn shift_ir_out(&mut self, ir: &BitSlice, pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftIR {
            ir: ir.to_owned(),
            capture: false,
            pause,
        });
    }
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
    fn shift_dr_out(&mut self, dr: &BitSlice, pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftDR {
            dr: dr.to_owned(),
            capture: true,
            pause,
        });
    }
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

    fn set_ir(&mut self, ir: &BitSlice) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::SetIR(ir.to_owned()));
    }
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
    fn write_reg(&mut self, ir: &BitSlice, dr: &BitSlice) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::WriteReg {
            ir: ir.to_owned(),
            dr: dr.to_owned(),
        });
    }
}
