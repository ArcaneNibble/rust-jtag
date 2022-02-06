use crate::*;

use bitvec::prelude::*;

/// Trait for JTAG adapters that are built around bit-banging one clock cycle
/// at a time
pub trait BitbangJTAGAdapter {
    /// Set the clock speed (i.e. the pulse width of one clock cycle)
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64;
    /// Shift one bit out on TMS/TDI. Capture the value of TDO.
    ///
    /// Note: This function assumes that the data capture occurs at _the end_
    /// of the cycle, i.e.
    /// <pre>
    /// TCK  _______|‾‾‾‾‾‾‾|_______
    /// TDI  ----╳ new val---------╳
    /// TMS  ----╳ new val---------╳
    /// TDO  ╳ prev val-----╳ new val
    /// capture                  ^
    /// </pre>
    /// This will be notably _different_ from the FTDI MPSSE implementation,
    /// i.e.
    /// <pre>
    /// TCK  _______|‾‾‾‾‾‾‾|_______
    /// TDI  ╳ prev val-----╳ new val
    /// TMS  ╳ prev val-----╳ new val
    /// TDO  ╳ prev val-----╳ new val
    /// capture     ^
    /// </pre>
    fn shift_one_bit(&mut self, tms: bool, tdi: bool) -> bool;
}

/// State required to be stored by a [BitbangJTAGAdapter] in order for
/// this crate to automatically implement [ChunkShifterJTAGAdapter] and
/// eventually [JTAGAdapter] for it.
#[derive(Clone, Debug)]
pub struct BitbangJTAGAdapterState {
    last_tdo: bool,
}
impl BitbangJTAGAdapterState {
    pub fn new() -> Self {
        Self {
            // We need this because of where we capture data and how
            // that interacts with shifting TDI/TDO
            //
            // FIXME: Explain why we do it the way we do
            last_tdo: false,
        }
    }
}

/// Automatically turn [BitbangJTAGAdapter] into a [ChunkShifterJTAGAdapter]
impl<T: BitbangJTAGAdapter + AsMut<BitbangJTAGAdapterState>> ChunkShifterJTAGAdapter for T {
    fn delay_ns(&mut self, ns: u64) -> u64 {
        std::thread::sleep(std::time::Duration::from_nanos(ns));
        ns
    }
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64 {
        BitbangJTAGAdapter::set_clk_speed(self, clk_hz)
    }

    fn shift_tms_chunk(&mut self, tms_chunk: &BitSlice) {
        for tms in tms_chunk {
            let tdo = self.shift_one_bit(*tms, false);
            // XXX this can be optimized maybe
            let state_data: &mut BitbangJTAGAdapterState = self.as_mut();
            state_data.last_tdo = tdo;
        }
    }
    fn shift_tdi_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) {
        for (i, tdi) in tdi_chunk.into_iter().enumerate() {
            let tdo = self.shift_one_bit(
                if tms_exit && i == tdi_chunk.len() - 1 {
                    true
                } else {
                    false
                },
                *tdi,
            );
            // XXX this can be optimized maybe
            let state_data: &mut BitbangJTAGAdapterState = self.as_mut();
            state_data.last_tdo = tdo;
        }
    }
    fn shift_tditdo_chunk(&mut self, tdi_chunk: &BitSlice, tms_exit: bool) -> BitVec {
        let mut ret: BitVec = BitVec::with_capacity(tdi_chunk.len());
        if tdi_chunk.len() > 0 {
            let state_data: &mut BitbangJTAGAdapterState = self.as_mut();
            ret.push(state_data.last_tdo);
        }

        for (i, tdi) in tdi_chunk.into_iter().enumerate() {
            let tdo = self.shift_one_bit(
                if tms_exit && i == tdi_chunk.len() - 1 {
                    true
                } else {
                    false
                },
                *tdi,
            );
            // XXX this can be optimized maybe
            let state_data: &mut BitbangJTAGAdapterState = self.as_mut();
            state_data.last_tdo = tdo;

            if i != tdi_chunk.len() - 1 {
                ret.push(tdo);
            }
        }
        ret
    }
}
