use bitvec::prelude::*;

mod fsm;
pub use fsm::JTAGState;

mod native;
pub use native::{JTAGAction, JTAGAdapter, JTAGAdapterState, JTAGOutput};

mod bitbang;
pub use bitbang::{BitbangJTAGAdapter, BitbangJTAGAdapterState};

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

pub trait StateTrackingJTAGAdapter {
    fn execute_stjtag_action(&mut self, action: &JTAGAction) -> JTAGOutput;
}

impl<T: StateTrackingJTAGAdapter + AsMut<JTAGAdapterState>> JTAGAdapter for T {
    fn execute_actions(&mut self, actions: &[JTAGAction]) -> Vec<JTAGOutput> {
        actions
            .into_iter()
            .map(|action| {
                match action {
                    JTAGAction::DelayNS(..)
                    | JTAGAction::SetClkSpeed(..)
                    | JTAGAction::ResetToTLR
                    | JTAGAction::GoViaStates(..)
                    | JTAGAction::ShiftBits { .. } => self.execute_stjtag_action(action),

                    JTAGAction::ShiftIR { ir, capture, pause } => {
                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftIR,
                        ]));

                        let ret = self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: ir.clone(),
                            capture: *capture,
                            tms_exit: true,
                        });

                        if *pause {
                            self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                                JTAGState::PauseIR,
                            ]));
                        } else {
                            self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                                JTAGState::RunTestIdle,
                            ]));
                        }

                        ret
                    }
                    JTAGAction::ShiftDR { dr, capture, pause } => {
                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftDR,
                        ]));

                        let ret = self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: dr.clone(),
                            capture: *capture,
                            tms_exit: true,
                        });

                        if *pause {
                            self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                                JTAGState::PauseDR,
                            ]));
                        } else {
                            self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                                JTAGState::RunTestIdle,
                            ]));
                        }

                        ret
                    }

                    // XXX need to track stuff for optimization here
                    JTAGAction::SetIR(ir) => {
                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftIR,
                        ]));

                        self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: ir.clone(),
                            capture: false,
                            tms_exit: true,
                        });

                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::RunTestIdle,
                        ]));

                        JTAGOutput::NoData
                    }
                    JTAGAction::ReadReg { ir, drlen } => {
                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftIR,
                        ]));

                        self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: ir.clone(),
                            capture: false,
                            tms_exit: true,
                        });

                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftDR,
                        ]));

                        let ret = self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: BitVec::repeat(false, *drlen),
                            capture: true,
                            tms_exit: true,
                        });

                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::RunTestIdle,
                        ]));

                        ret
                    }
                    JTAGAction::WriteReg { ir, dr } => {
                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftIR,
                        ]));

                        self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: ir.clone(),
                            capture: false,
                            tms_exit: true,
                        });

                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::ShiftDR,
                        ]));

                        self.execute_stjtag_action(&JTAGAction::ShiftBits {
                            bits_tdi: dr.clone(),
                            capture: false,
                            tms_exit: true,
                        });

                        self.execute_stjtag_action(&JTAGAction::GoViaStates(vec![
                            JTAGState::RunTestIdle,
                        ]));

                        JTAGOutput::NoData
                    }
                }
            })
            .collect()
    }
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

#[cfg(test)]
mod tests;

pub mod drivers;
