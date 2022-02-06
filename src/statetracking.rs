use crate::*;

use bitvec::prelude::*;

/// Trait for JTAG adapters that can automatically follow the JTAG state
/// machine
pub trait StateTrackingJTAGAdapter {
    /// Execute the given JTAG action and return its result.
    ///
    /// The following actions are valid for this type of adapter:
    /// * [JTAGAction::DelayNS]
    /// * [JTAGAction::SetClkSpeed]
    /// * [JTAGAction::ResetToTLR]
    /// * [JTAGAction::GoViaStates]
    /// * [JTAGAction::ShiftBits]
    fn execute_stjtag_action(&mut self, action: &JTAGAction) -> JTAGOutput;
}

/// Automatically turn [ChunkShifterJTAGAdapter] into a [StateTrackingJTAGAdapter]
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
