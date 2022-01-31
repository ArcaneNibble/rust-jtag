use bitvec::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
/// Represents a state in the JTAG state machine
pub enum JTAGState {
    TestLogicReset,
    RunTestIdle,

    SelectDR,
    CaptureDR,
    ShiftDR,
    Exit1DR,
    PauseDR,
    Exit2DR,
    UpdateDR,

    SelectIR,
    CaptureIR,
    ShiftIR,
    Exit1IR,
    PauseIR,
    Exit2IR,
    UpdateIR,
}

impl JTAGState {
    /// Calculate the shortest path (via TMS transitions) to get to `end` from this state
    pub fn path_to(self: &JTAGState, end: JTAGState) -> &'static BitSlice {
        match self {
            JTAGState::TestLogicReset => match end {
                JTAGState::TestLogicReset => {
                    bits![static]
                }
                JTAGState::RunTestIdle => {
                    bits![static 0]
                }
                JTAGState::SelectDR => {
                    bits![static 0, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 0, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 0, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 0, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 0, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 0, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 0, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 0, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 0, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 0, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 0, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 0, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 0, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::RunTestIdle => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static]
                }
                JTAGState::SelectDR => {
                    bits![static 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 0, 1, 1]
                }
            },
            JTAGState::SelectDR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static]
                }
                JTAGState::CaptureDR => {
                    bits![static 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 0, 1, 1]
                }
            },
            JTAGState::CaptureDR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static]
                }
                JTAGState::ShiftDR => {
                    bits![static 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::ShiftDR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static]
                }
                JTAGState::Exit1DR => {
                    bits![static 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::Exit1DR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static]
                }
                JTAGState::PauseDR => {
                    bits![static 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::PauseDR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static]
                }
                JTAGState::Exit2DR => {
                    bits![static 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::Exit2DR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static]
                }
                JTAGState::UpdateDR => {
                    bits![static 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 1, 0, 1, 1]
                }
            },
            JTAGState::UpdateDR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1, 0, 1, 1]
                }
            },
            JTAGState::SelectIR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 0, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 0, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 0, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 0, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static]
                }
                JTAGState::CaptureIR => {
                    bits![static 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 0, 1, 1]
                }
            },
            JTAGState::CaptureIR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 1, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static]
                }
                JTAGState::ShiftIR => {
                    bits![static 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1]
                }
            },
            JTAGState::ShiftIR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 1, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static]
                }
                JTAGState::Exit1IR => {
                    bits![static 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1]
                }
            },
            JTAGState::Exit1IR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static]
                }
                JTAGState::PauseIR => {
                    bits![static 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1]
                }
            },
            JTAGState::PauseIR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 1, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 1, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static]
                }
                JTAGState::Exit2IR => {
                    bits![static 1]
                }
                JTAGState::UpdateIR => {
                    bits![static 1, 1]
                }
            },
            JTAGState::Exit2IR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 1, 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static]
                }
                JTAGState::UpdateIR => {
                    bits![static 1]
                }
            },
            JTAGState::UpdateIR => match end {
                JTAGState::TestLogicReset => {
                    bits![static 1, 1, 1]
                }
                JTAGState::RunTestIdle => {
                    bits![static 0]
                }
                JTAGState::SelectDR => {
                    bits![static 1]
                }
                JTAGState::CaptureDR => {
                    bits![static 1, 0]
                }
                JTAGState::ShiftDR => {
                    bits![static 1, 0, 0]
                }
                JTAGState::Exit1DR => {
                    bits![static 1, 0, 1]
                }
                JTAGState::PauseDR => {
                    bits![static 1, 0, 1, 0]
                }
                JTAGState::Exit2DR => {
                    bits![static 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateDR => {
                    bits![static 1, 0, 1, 1]
                }
                JTAGState::SelectIR => {
                    bits![static 1, 1]
                }
                JTAGState::CaptureIR => {
                    bits![static 1, 1, 0]
                }
                JTAGState::ShiftIR => {
                    bits![static 1, 1, 0, 0]
                }
                JTAGState::Exit1IR => {
                    bits![static 1, 1, 0, 1]
                }
                JTAGState::PauseIR => {
                    bits![static 1, 1, 0, 1, 0]
                }
                JTAGState::Exit2IR => {
                    bits![static 1, 1, 0, 1, 0, 1]
                }
                JTAGState::UpdateIR => {
                    bits![static]
                }
            },
        }
    }

    /// Calculate the next state from this state with the given value on TMS
    pub const fn transition(self: &JTAGState, tms: bool) -> JTAGState {
        match &self {
            JTAGState::TestLogicReset => {
                if !tms {
                    JTAGState::RunTestIdle
                } else {
                    JTAGState::TestLogicReset
                }
            }
            JTAGState::RunTestIdle => {
                if !tms {
                    JTAGState::RunTestIdle
                } else {
                    JTAGState::SelectDR
                }
            }

            JTAGState::SelectDR => {
                if !tms {
                    JTAGState::CaptureDR
                } else {
                    JTAGState::SelectIR
                }
            }
            JTAGState::CaptureDR => {
                if !tms {
                    JTAGState::ShiftDR
                } else {
                    JTAGState::Exit1DR
                }
            }
            JTAGState::ShiftDR => {
                if !tms {
                    JTAGState::ShiftDR
                } else {
                    JTAGState::Exit1DR
                }
            }
            JTAGState::Exit1DR => {
                if !tms {
                    JTAGState::PauseDR
                } else {
                    JTAGState::UpdateDR
                }
            }
            JTAGState::PauseDR => {
                if !tms {
                    JTAGState::PauseDR
                } else {
                    JTAGState::Exit2DR
                }
            }
            JTAGState::Exit2DR => {
                if !tms {
                    JTAGState::ShiftDR
                } else {
                    JTAGState::UpdateDR
                }
            }
            JTAGState::UpdateDR => {
                if !tms {
                    JTAGState::RunTestIdle
                } else {
                    JTAGState::SelectDR
                }
            }

            JTAGState::SelectIR => {
                if !tms {
                    JTAGState::CaptureIR
                } else {
                    JTAGState::TestLogicReset
                }
            }
            JTAGState::CaptureIR => {
                if !tms {
                    JTAGState::ShiftIR
                } else {
                    JTAGState::Exit1IR
                }
            }
            JTAGState::ShiftIR => {
                if !tms {
                    JTAGState::ShiftIR
                } else {
                    JTAGState::Exit1IR
                }
            }
            JTAGState::Exit1IR => {
                if !tms {
                    JTAGState::PauseIR
                } else {
                    JTAGState::UpdateIR
                }
            }
            JTAGState::PauseIR => {
                if !tms {
                    JTAGState::PauseIR
                } else {
                    JTAGState::Exit2IR
                }
            }
            JTAGState::Exit2IR => {
                if !tms {
                    JTAGState::ShiftIR
                } else {
                    JTAGState::UpdateIR
                }
            }
            JTAGState::UpdateIR => {
                if !tms {
                    JTAGState::RunTestIdle
                } else {
                    JTAGState::SelectDR
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JTAGState;

    #[test]
    fn test_fsm_transitions() {
        for state_a in [
            JTAGState::TestLogicReset,
            JTAGState::RunTestIdle,
            JTAGState::SelectDR,
            JTAGState::CaptureDR,
            JTAGState::ShiftDR,
            JTAGState::Exit1DR,
            JTAGState::PauseDR,
            JTAGState::Exit2DR,
            JTAGState::UpdateDR,
            JTAGState::SelectIR,
            JTAGState::CaptureIR,
            JTAGState::ShiftIR,
            JTAGState::Exit1IR,
            JTAGState::PauseIR,
            JTAGState::Exit2IR,
            JTAGState::UpdateIR,
        ] {
            for state_b in [
                JTAGState::TestLogicReset,
                JTAGState::RunTestIdle,
                JTAGState::SelectDR,
                JTAGState::CaptureDR,
                JTAGState::ShiftDR,
                JTAGState::Exit1DR,
                JTAGState::PauseDR,
                JTAGState::Exit2DR,
                JTAGState::UpdateDR,
                JTAGState::SelectIR,
                JTAGState::CaptureIR,
                JTAGState::ShiftIR,
                JTAGState::Exit1IR,
                JTAGState::PauseIR,
                JTAGState::Exit2IR,
                JTAGState::UpdateIR,
            ] {
                let path = state_a.path_to(state_b);

                let mut state = state_a;
                for tms in path {
                    state = state.transition(*tms);
                }
                assert_eq!(state, state_b);
            }
        }
    }
}
