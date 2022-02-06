//! Built-in JTAG adapter drivers

mod crabbytty_prealpha;
pub use crabbytty_prealpha::CrabbyTTYPreAlphaJTAG;

mod ftdi_mpsse_jtag;
pub use ftdi_mpsse_jtag::FTDIJTAG;
