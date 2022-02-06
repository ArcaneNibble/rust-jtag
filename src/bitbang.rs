use crate::*;

use bitvec::prelude::*;

pub trait BitbangJTAGAdapter {
    fn set_clk_speed(&mut self, clk_hz: u64) -> u64;
    fn shift_one_bit(&mut self, tms: bool, tdi: bool) -> bool;
}

#[derive(Clone, Debug)]
pub struct BitbangJTAGAdapterState {
    last_tdo: bool,
}
impl BitbangJTAGAdapterState {
    pub fn new() -> Self {
        Self {
            // FIXME: document wtf is going on here
            last_tdo: false,
        }
    }
}

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
