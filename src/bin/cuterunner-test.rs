use jtag::JTAGAdapter;

fn main() {
    println!("Hello world!");

    let mut adapter = jtag::drivers::CrabbyTTYPreAlphaJTAG::new();
    adapter.reset_to_tlr();
    adapter.go_rti();

    let idcode = adapter.shift_dr_inout(&[false; 32], false);
    println!("idcode {idcode:?}");
}
