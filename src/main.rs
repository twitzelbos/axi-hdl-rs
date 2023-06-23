use axi_hdl_rs::AXI4LiteSlaveController;
use rust_hdl::prelude::*;

fn main() {
    let mut uut = AXI4LiteSlaveController::<32, 4>::default();
    uut.connect_all();
    println!("{}", generate_verilog(&uut));
}

#[test]
fn test_axi4lite_slave_controller_default() {
    let mut sim = simple_sim!(
        AXI4LiteSlaveController::<32, 4>,
        axi_bus.ACLK,
        100_000_000,
        ep,
        {
            let mut x = ep.init()?; // Get the circuit

            // start not in reset
            x.axi_bus.ARESETn.next = true;

            sim_assert!(ep, x.axi_bus.AWREADY.val(), x);
            sim_assert!(ep, x.axi_bus.ARREADY.val(), x);

            wait_clock_cycle!(ep, axi_bus.ACLK, x);
            // asserting reset is useless, because its probably the default state, but lets do it anyway
            x.axi_bus.ARESETn.next = false;

            // Spend one cycle in reset
            wait_clock_cycle!(ep, axi_bus.ACLK, x);
            sim_assert_eq!(ep, x.axi_bus.AWREADY.val(), false, x);
            sim_assert_eq!(ep, x.axi_bus.ARREADY.val(), false, x);

            // Come out of the reset
            x.axi_bus.ARESETn.next = true;

            wait_clock_cycle!(ep, axi_bus.ACLK, x);

            sim_assert!(ep, x.axi_bus.AWREADY.val(), x);
            sim_assert!(ep, x.axi_bus.ARREADY.val(), x);

            ep.done(x)
        }
    );

    // Run the simulation
    sim.run_to_file(
        AXI4LiteSlaveController::<32, 4>::default().into(),
        sim_time::ONE_MICROSECOND,
        &vcd_path!("axi4lite_slave_controller_default.vcd"),
    )
    .unwrap();
    vcd_to_svg(
        &vcd_path!("axi4lite_slave_controller_default.vcd"),
        "axi4lite_slave_controller_default.svg",
        &[
            "uut.axi_bus.ACLK",
            "uut.axi_bus.ARESETn",
            "uut.axi_bus.AWREADY",
            "uut.axi_bus.ARREADY",
        ],
        0,
        35 * sim_time::ONE_NANOSECOND,
    )
    .unwrap()
}

#[test]
fn test_axi4lite_slave_controller_read() {
    let mut sim = simple_sim!(
        AXI4LiteSlaveController::<32, 4>,
        axi_bus.ACLK,
        100_000_000,
        ep,
        {
            let mut x = ep.init()?; // Get the circuit

            // start not in reset
            x.axi_bus.ARESETn.next = true;
            //
            wait_clock_cycle!(ep, axi_bus.ACLK, x);
            x.axi_bus.ARADDR.next = 0x0000_0004.into();
            x.axi_bus.ARVALID.next = true;
            wait_clock_cycle!(ep, axi_bus.ACLK, x);
            sim_assert_eq!(ep, x.axi_bus.ARADDR.val(), 0x0000_0004, x);
            wait_clock_cycles!(ep, axi_bus.ACLK, x, 20);

            ep.done(x)
        }
    );

    // Run the simulation
    sim.run_to_file(
        AXI4LiteSlaveController::<32, 4>::default().into(),
        sim_time::ONE_MICROSECOND,
        &vcd_path!("axi4lite_slave_controller_read.vcd"),
    )
    .unwrap();
    vcd_to_svg(
        &vcd_path!("axi4lite_slave_controller_read.vcd"),
        "axi4lite_slave_controller_read.svg",
        &[
            "uut.axi_bus.ACLK",
            "uut.axi_bus.ARESETn",
            "uut.axi_bus.ARADDR",
            "uut.axi_bus.AWREADY",
            "uut.axi_bus.ARREADY",
            "uut.axi_bus.ARVALID",
            "uut.axi_bus.RREADY",
            "uut.axi_bus.RDATA",
        ],
        0,
        100 * sim_time::ONE_NANOSECOND,
    )
    .unwrap()
}
