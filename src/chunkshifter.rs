use crate::*;

#[derive(Clone, Debug)]
pub struct ChunkShifterJTAGAdapterState {
    current_state: JTAGState,
}
impl ChunkShifterJTAGAdapterState {
    pub fn new() -> Self {
        Self {
            // FIXME?
            current_state: JTAGState::TestLogicReset,
        }
    }
}

pub trait ChunkShifterJTAGAdapter {
    fn delay_ns(&mut self, ns: u64) -> u64;
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64;

    fn shift_tms_chunk(&mut self, tms_chunk: &BitSlice);
    fn shift_tdi_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool);
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) -> BitVec;
}

impl<T: ChunkShifterJTAGAdapter + AsMut<ChunkShifterJTAGAdapterState>> StateTrackingJTAGAdapter
    for T
{
    fn execute_stjtag_action(&mut self, action: &JTAGAction) -> JTAGOutput {
        match action {
            JTAGAction::ShiftIR { .. }
            | JTAGAction::ShiftDR { .. }
            | JTAGAction::SetIR { .. }
            | JTAGAction::ReadReg { .. }
            | JTAGAction::WriteReg { .. } => {
                unreachable!()
            }

            JTAGAction::DelayNS(ns) => {
                let actual_ns = self.delay_ns(*ns);
                JTAGOutput::ActualDelay(actual_ns)
            }
            JTAGAction::SetClkSpeed(clk_hz) => {
                let actual_clk = self.set_clk_speed(*clk_hz);
                JTAGOutput::ActualClkSpeed(actual_clk)
            }
            JTAGAction::ResetToTLR => {
                self.shift_tms_chunk(bits![1; 5]);
                JTAGOutput::NoData
            }
            JTAGAction::GoViaStates(jtag_states) => {
                let state_data: &mut ChunkShifterJTAGAdapterState = self.as_mut();
                let mut prev_state = state_data.current_state;
                let mut path = BitVec::new();

                for jtag_state in jtag_states {
                    let pathelem = prev_state.path_to(*jtag_state);
                    path.extend_from_bitslice(pathelem);
                    prev_state = *jtag_state;
                }

                self.shift_tms_chunk(&path);

                let state_data: &mut ChunkShifterJTAGAdapterState = self.as_mut();
                state_data.current_state = prev_state;

                JTAGOutput::NoData
            }
            JTAGAction::ShiftBits {
                bits_tdi,
                capture,
                tms_exit,
            } => {
                let state_data: &mut ChunkShifterJTAGAdapterState = self.as_mut();
                state_data.current_state = state_data.current_state.transition(*tms_exit);

                if *capture {
                    let ret = self.shift_tditdo_chunk(bits_tdi, *tms_exit);
                    JTAGOutput::CapturedBits(ret)
                } else {
                    self.shift_tdi_chunk(bits_tdi, *tms_exit);
                    JTAGOutput::NoData
                }
            }
        }
    }
}
