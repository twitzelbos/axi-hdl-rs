use axi_hdl_rs::AXI4LiteSlaveController;
use rust_hdl::prelude::*;

fn main() {
    let mut uut = AXI4LiteSlaveController::<32>::default();
    uut.connect_all();
    println!("{}", generate_verilog(&uut));
}

#[test]
fn test_axi4lite_slave_controller_default() {
    let mut sim = simple_sim!(
        AXI4LiteSlaveController::<32>,
        axi_bus.ACLK,
        100_000_000,
        ep,
        {
            let mut x = ep.init()?; // Get the circuit

            // Come out of the reset
            x.axi_bus.ARESETn.next = true;

            wait_clock_cycle!(ep, axi_bus.ACLK, x);

            sim_assert!(ep, x.axi_bus.AWREADY.val(), x);

            ep.done(x)
        }
    );

    // Run the simulation
    sim.run(
        AXI4LiteSlaveController::<32>::default().into(),
        sim_time::ONE_MICROSECOND,
    )
    .unwrap();
}

fn test_axi4lite_slave_controller_read() {
    let mut sim = simple_sim!(
        AXI4LiteSlaveController::<32>,
        axi_bus.ACLK,
        100_000_000,
        ep,
        {
            let mut x = ep.init()?; // Get the circuit

            //
            x.axi_bus.ARADDR.next = 0x0000_0000.into();

            wait_clock_cycle!(ep, axi_bus.ACLK, x);

            ep.done(x)
        }
    );

    // Run the simulation
    sim.run(
        AXI4LiteSlaveController::<32>::default().into(),
        sim_time::ONE_MICROSECOND,
    )
    .unwrap();
}
