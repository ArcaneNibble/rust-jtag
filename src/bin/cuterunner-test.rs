use jtag::JTAGAdapter;

fn main() {
    println!("Hello world!");

    let mut adapter = jtag::drivers::CrabbyTTYPreAlphaJTAG::new();
    adapter.reset_to_tlr();
    adapter.go_rti();

    let idcode = adapter.shift_dr_inout(&[false; 32], false);

    let mut idcode_ = 0u32;
    for (i, bit) in idcode.into_iter().enumerate() {
        if bit {
            idcode_ |= 1 << i;
        }
    }

    println!("idcode {idcode_:08X}");
}
