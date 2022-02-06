use crate::*;

use bitvec::prelude::*;

/// Trait for JTAG adapters that are built around shifting blocks of bits
/// at a time (e.g. FTDI MPSSE, Xilinx Platform Cable USB)
pub trait ChunkShifterJTAGAdapter {
    /// Wait for a given number of nanoseconds
    fn delay_ns(&mut self, ns: u64) -> u64;
    /// Set the JTAG clock speed in Hz
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64;

    /// Shift the given bits out on TMS
    fn shift_tms_chunk(&mut self, tms_chunk: &BitSlice);
    /// Shift the given bits out on TDI. If `tms_exit` is `true`,
    /// the final bit will be shifted with TMS=1 (which would cause a
    /// transition from Shift-IR/DR to Exit1-IR/DR). Otherwise, the final bit
    /// will be shifted with TMS=0.
    fn shift_tdi_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool);
    /// Shift the given bits out on TDI. Capture data on TDO.
    /// If `tms_exit` is `true`,
    /// the final bit will be shifted with TMS=1 (which would cause a
    /// transition from Shift-IR/DR to Exit1-IR/DR). Otherwise, the final bit
    /// will be shifted with TMS=0.
    ///
    // FIXME: Explain how the timing here is "behind" and different from
    // the bitbang mode
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) -> BitVec;
}

/// State required to be stored by a [ChunkShifterJTAGAdapter] in order for
/// this crate to automatically implement [StateTrackingJTAGAdapter] and
/// eventually [JTAGAdapter] for it.
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

/// Automatically turn [ChunkShifterJTAGAdapter] into a [StateTrackingJTAGAdapter]
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
