use axi_hdl_rs::AXI4LiteSlaveController;
use rust_hdl::prelude::*;

fn main() {
    let mut uut = AXI4LiteSlaveController::<32>::default();
    uut.connect_all();
    println!("{}", generate_verilog(&uut));
}
