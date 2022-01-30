#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
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

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
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
        bits_tdi: Vec<bool>,
        capture: bool,
        tms_exit: bool,
    },

    /// Take the TAP from its current state to Shift-IR, then shift in the
    /// data from [ir][Self::ShiftIR::ir] while capturing data if
    /// [capture][Self::ShiftIR::capture] is `true`.
    /// If [pause][Self::ShiftIR::pause] is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-IR.
    ShiftIR {
        ir: Vec<bool>,
        capture: bool,
        pause: bool,
    },
    /// Take the TAP from its current state to Shift-DR, then shift in the
    /// data from [dr][Self::ShiftDR::dr] while capturing data if
    /// [capture][Self::ShiftDR::capture] is `true`.
    /// If [pause][Self::ShiftDR::pause] is `false`, then take the TAP to
    /// Run-Test/Idle. Otherwise, take the TAP to Pause-DR.
    ShiftDR {
        dr: Vec<bool>,
        capture: bool,
        pause: bool,
    },

    /// If the current IR value is not equal to this value, take the TAP to
    /// Shift-IR, shift in this value, and then take the TAP to Run-Test/Idle.
    /// If the current IR value is equal, do nothing.
    SetIR(Vec<bool>),
    /// Set the current IR to this value if it is not already, and then
    /// take the TAP to Shift-DR. Capture the data, and then take the TAP to
    /// Run-Test/Idle.
    ReadReg { ir: Vec<bool>, drlen: usize },
    /// Set the current IR to [ir][Self::WriteReg::ir] if it is not already,
    /// and then take the TAP to Shift-DR. Shift in the data from
    /// [dr][Self::WriteReg::dr], and then take the TAP to Run-Test/Idle.
    WriteReg { ir: Vec<bool>, dr: Vec<bool> },
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum JTAGOutput {
    /// No data was stored for the corresponding JTAG action
    NoData,
    /// Captured TDO data for the corresponding JTAG action
    CapturedBits(Vec<bool>),
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

    fn shift_bits_out(&mut self, bits: &[bool], tms_exit: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftBits {
            bits_tdi: bits.to_vec(),
            capture: false,
            tms_exit,
        });
    }
    fn shift_bits_inout(&mut self, bits: &[bool], tms_exit: bool) -> Vec<bool> {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftBits {
            bits_tdi: bits.to_vec(),
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

    fn shift_ir_out(&mut self, ir: &[bool], pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftIR {
            ir: ir.to_vec(),
            capture: false,
            pause,
        });
    }
    fn shift_ir_inout(&mut self, ir: &[bool], pause: bool) -> Vec<bool> {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftIR {
            ir: ir.to_vec(),
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
    fn shift_dr_out(&mut self, dr: &[bool], pause: bool) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftDR {
            dr: dr.to_vec(),
            capture: true,
            pause,
        });
    }
    fn shift_dr_inout(&mut self, dr: &[bool], pause: bool) -> Vec<bool> {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ShiftDR {
            dr: dr.to_vec(),
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

    fn set_ir(&mut self, ir: &[bool]) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::SetIR(ir.to_vec()));
    }
    fn read_reg(&mut self, ir: &[bool], drlen: usize) -> Vec<bool> {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::ReadReg {
            ir: ir.to_vec(),
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
    fn write_reg(&mut self, ir: &[bool], dr: &[bool]) {
        let state: &mut JTAGAdapterState = self.as_mut();
        state.queued_actions.push(JTAGAction::WriteReg {
            ir: ir.to_vec(),
            dr: dr.to_vec(),
        });
    }
}

pub trait BitbangJTAGAdapter {
    fn set_clk_speed(&mut self, clk_hz: u64);
    fn shift_one_bit(&mut self, tms: bool, tdi: bool) -> bool;
}

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
    fn delay_ns(&mut self, ns: u64);
    fn set_clk_speed(&mut self, clk_hz: u64);

    fn shift_tms_chunk(&mut self, tms_chunk: &[bool]);
    fn shift_tdi_chunk(&mut self, tdi_chunk: &[bool], tms_exit: bool);
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &[bool], tms_exit: bool) -> Vec<bool>;
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
                            bits_tdi: vec![false; *drlen],
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
                self.delay_ns(*ns);
                JTAGOutput::NoData
            }
            JTAGAction::SetClkSpeed(clk_hz) => {
                self.set_clk_speed(*clk_hz);
                JTAGOutput::NoData
            }
            JTAGAction::ResetToTLR => {
                self.shift_tms_chunk(&[true; 5]);
                JTAGOutput::NoData
            }
            JTAGAction::GoViaStates(_) => {
                todo!()
            }
            JTAGAction::ShiftBits {
                bits_tdi,
                capture,
                tms_exit,
            } => {
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

impl<T: BitbangJTAGAdapter> ChunkShifterJTAGAdapter for T {
    fn delay_ns(&mut self, ns: u64) {
        std::thread::sleep(std::time::Duration::from_nanos(ns))
    }
    fn set_clk_speed(&mut self, clk_hz: u64) {
        BitbangJTAGAdapter::set_clk_speed(self, clk_hz);
    }

    fn shift_tms_chunk(&mut self, tms_chunk: &[bool]) {
        for tms in tms_chunk {
            self.shift_one_bit(*tms, false);
        }
    }
    fn shift_tdi_chunk(&mut self, tdi_chunk: &[bool], tms_exit: bool) {
        for (i, tdi) in tdi_chunk.into_iter().enumerate() {
            self.shift_one_bit(
                if tms_exit && i == tdi_chunk.len() - 1 {
                    true
                } else {
                    false
                },
                *tdi,
            );
        }
    }
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &[bool], tms_exit: bool) -> Vec<bool> {
        let mut ret = Vec::with_capacity(tdi_chunk.len());
        for (i, tdi) in tdi_chunk.into_iter().enumerate() {
            let tdo = self.shift_one_bit(
                if tms_exit && i == tdi_chunk.len() - 1 {
                    true
                } else {
                    false
                },
                *tdi,
            );
            ret.push(tdo);
        }
        ret
    }
}

#[cfg(test)]
mod tests;
