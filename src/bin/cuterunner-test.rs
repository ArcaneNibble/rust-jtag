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

    let idcode2 = adapter.shift_dr_inout(&[false; 32], false);

    let mut idcode2_ = 0u32;
    for (i, bit) in idcode2.into_iter().enumerate() {
        if bit {
            idcode2_ |= 1 << i;
        }
    }

    println!("idcode2 {idcode2_:08X}");

    adapter.go_shiftdr();
    let idcode3_a = adapter.shift_bits_inout(&[false; 16], false);
    let idcode3_b = adapter.shift_bits_inout(&[false; 16], true);
    adapter.go_rti();
    adapter.flush();

    let mut idcode3 = 0u32;
    for (i, bit) in idcode3_a.into_iter().enumerate() {
        if bit {
            idcode3 |= 1 << i;
        }
    }
    for (i, bit) in idcode3_b.into_iter().enumerate() {
        if bit {
            idcode3 |= 1 << (i + 16);
        }
    }

    println!("idcode3 {idcode3:08X}");
}
