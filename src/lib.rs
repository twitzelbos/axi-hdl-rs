#![allow(non_snake_case, non_camel_case_types)]

use rust_hdl::prelude::*;

pub enum AXI4Response {
    OKAY = 0b00, // Normal access okay indicates if a normal access has been successful. Can also indicate an exclusive access failure.
    EXOKAY = 0b01, // Exclusive access okay indicates that either the read or write portion of an exclusive access has been successful.
    SLVERR = 0b10, // Slave error is used when the access has reached the slave successfully, but the slave wishes to return an error condition to the originating master.
    DECERR = 0b11, // Decode error is generated typically by an interconnect component to indicate that there is no slave at the transaction address.
}

pub enum AXI4Burst {
    FIXED = 0b00,
    INCR = 0b01,
    WRAP = 0b10,
    RESERVED = 0b11,
}

pub enum AXI4Cache {
    DEVICE_NON_BUFFERABLE = 0b0000,
    DEVICE_BUFFERABLE = 0b0001,
    NORMAL_NON_CACHEABLE_NON_BUFFERABLE = 0b0010,
    NORMAL_NON_CACHEABLE_BUFFERABLE = 0b0011,
}

// According to the AXI4 standard, the data width can be 8, 16, 32, 64, 128, 256, 512, or 1024 bits.

#[derive(LogicInterface)]
pub struct AXI4Master<const DW: usize> {
    // global signals
    pub ACLK: Signal<In, Clock>,
    pub ARESETn: Signal<In, Bit>,

    // write address channel signals
    pub AWID: Signal<Out, Bits<4>>,
    pub AWADDR: Signal<Out, Bits<32>>,
    pub AWLEN: Signal<Out, Bits<4>>,
    pub AWSIZE: Signal<Out, Bits<3>>,
    pub AWBURST: Signal<Out, Bits<2>>,
    pub AWLOCK: Signal<Out, Bits<1>>,
    pub AWCACHE: Signal<Out, Bits<4>>,
    pub AWPROT: Signal<Out, Bits<3>>,
    pub AWVALID: Signal<Out, Bit>,
    pub AWREADY: Signal<In, Bit>,

    // write data channel signals
    pub WID: Signal<Out, Bits<4>>,
    pub WDATA: Signal<Out, Bits<DW>>,
    pub WSTRB: Signal<Out, Bits<4>>,
    pub WLAST: Signal<Out, Bit>,
    pub WVALID: Signal<Out, Bit>,
    pub WREADY: Signal<In, Bit>,

    // write response channel signals
    pub BID: Signal<In, Bits<4>>,
    pub BRESP: Signal<In, Bits<2>>,
    pub BVALID: Signal<In, Bit>,
    pub BREADY: Signal<Out, Bit>,

    // read address channel signals
    pub ARID: Signal<Out, Bits<4>>,
    pub ARADDR: Signal<Out, Bits<32>>,
    pub ARLEN: Signal<Out, Bits<4>>,
    pub ARSIZE: Signal<Out, Bits<3>>,
    pub ARBURST: Signal<Out, Bits<2>>,
    pub ARLOCK: Signal<Out, Bits<2>>,
    pub ARCACHE: Signal<Out, Bits<4>>,
    pub ARPROT: Signal<Out, Bits<3>>,
    pub ARVALID: Signal<Out, Bit>,
    pub ARREADY: Signal<In, Bit>,

    // read data channel signals
    pub RID: Signal<In, Bits<4>>,
    pub RDATA: Signal<In, Bits<DW>>,
    pub RRESP: Signal<In, Bits<2>>,
    pub RLAST: Signal<In, Bit>,
    pub RVALID: Signal<In, Bit>,
    pub RREADY: Signal<Out, Bit>,
}

#[derive(LogicInterface, Default)]
pub struct AXI4Slave<const DW: usize> {
    // global signals
    pub ACLK: Signal<In, Clock>,
    pub ARESETn: Signal<In, Bit>,

    // write address channel signals
    pub AWID: Signal<In, Bits<4>>,
    pub AWADDR: Signal<In, Bits<32>>,
    pub AWLEN: Signal<In, Bits<4>>,
    pub AWSIZE: Signal<In, Bits<3>>,
    pub AWBURST: Signal<In, Bits<2>>,
    pub AWLOCK: Signal<In, Bits<1>>,
    pub AWCACHE: Signal<In, Bits<4>>,
    pub AWPROT: Signal<In, Bits<3>>,
    pub AWVALID: Signal<In, Bit>,
    pub AWREADY: Signal<Out, Bit>,

    // write data channel signals
    pub WID: Signal<In, Bits<4>>,
    pub WDATA: Signal<In, Bits<DW>>,
    pub WSTRB: Signal<In, Bits<4>>,
    pub WLAST: Signal<In, Bit>,
    pub WVALID: Signal<In, Bit>,
    pub WREADY: Signal<Out, Bit>,

    // write response channel signals
    pub BID: Signal<Out, Bits<4>>,
    pub BRESP: Signal<Out, Bits<2>>,
    pub BVALID: Signal<Out, Bit>,
    pub BREADY: Signal<In, Bit>,

    // read address channel signals
    pub ARID: Signal<In, Bits<4>>,
    pub ARADDR: Signal<In, Bits<32>>,
    pub ARLEN: Signal<In, Bits<4>>,
    pub ARSIZE: Signal<In, Bits<3>>,
    pub ARBURST: Signal<In, Bits<2>>,
    pub ARLOCK: Signal<In, Bits<2>>,
    pub ARCACHE: Signal<In, Bits<4>>,
    pub ARPROT: Signal<In, Bits<3>>,
    pub ARVALID: Signal<In, Bit>,
    pub ARREADY: Signal<Out, Bit>,

    // read data channel signals
    pub RID: Signal<Out, Bits<4>>,
    pub RDATA: Signal<Out, Bits<DW>>,
    pub RRESP: Signal<Out, Bits<2>>,
    pub RLAST: Signal<Out, Bit>,
    pub RVALID: Signal<Out, Bit>,
    pub RREADY: Signal<In, Bit>,
}

// AXI4Lite slaves only support 32-bit and 64-bit data widths, but won't be enforcing that here.
#[derive(LogicBlock)]
pub struct AXI4LiteSlaveController<const DW: usize, const N: usize> {
    pub axi_bus: AXI4Slave<DW>,

    // local signals
    aclk: Signal<Local, Clock>,
    reg_rden: Signal<Local, Bit>,
    read_data: Signal<Local, Bits<DW>>,

    // registers
    awready: DFFWithInit<Bit>,
    wready: DFF<Bit>,
    bid: DFF<Bits<4>>,
    bresp: DFF<Bits<2>>,
    bvalid: DFF<Bit>,

    arready: DFFWithInit<Bit>,

    rid: DFF<Bits<4>>,
    rresp: DFF<Bits<2>>,
    rlast: DFF<Bit>,
    rvalid: DFF<Bit>,
    rdata: DFF<Bits<DW>>,

    // internal address registers
    axi_raddr: DFF<Bits<32>>,
    axi_waddr: DFF<Bits<32>>,
    axi_rid: DFF<Bits<4>>,
    axi_wid: DFF<Bits<4>>,

    // for testing, 4 registers
    pub reg0: DFF<Bits<DW>>,
    pub reg1: DFFWithInit<Bits<DW>>,
    pub reg2: DFF<Bits<DW>>,
    pub reg3: DFF<Bits<DW>>,

    pub registers: [DFF<Bits<DW>>; N],
}

impl<const DW: usize, const N: usize> Default for AXI4LiteSlaveController<DW, N> {
    fn default() -> Self {
        Self {
            arready: DFFWithInit::new(true),
            awready: DFFWithInit::new(true),
            wready: Default::default(),
            bid: Default::default(),
            bresp: Default::default(),
            bvalid: Default::default(),
            rid: Default::default(),
            rresp: Default::default(),
            rlast: Default::default(),
            rvalid: Default::default(),
            rdata: Default::default(),
            axi_raddr: Default::default(),
            axi_waddr: Default::default(),
            axi_rid: Default::default(),
            axi_wid: Default::default(),
            reg0: Default::default(),
            reg1: DFFWithInit::new(0x1234_5678.into()),
            reg2: Default::default(),
            reg3: Default::default(),
            aclk: Default::default(),
            reg_rden: Default::default(),
            read_data: Default::default(),
            axi_bus: Default::default(),
            registers: array_init::array_init(|_| Default::default()),
        }
    }
}

impl<const DW: usize, const N: usize> Logic for AXI4LiteSlaveController<DW, N> {
    #[hdl_gen]
    fn update(&mut self) {
        self.aclk.next = self.axi_bus.ACLK.val();

        dff_setup!(
            self, aclk, awready, wready, bid, bresp, bvalid, arready, rvalid, rdata, rid, rresp,
            rlast, reg0, reg1, reg2, reg3, axi_raddr, axi_waddr, axi_rid, axi_wid
        );

        for i in 0..N {
            self.registers[i].clock.next = self.aclk.val();
            self.registers[i].d.next = self.registers[i].q.val();
        }

        // assign all the output registers
        self.axi_bus.AWREADY.next = self.awready.q.val();
        self.axi_bus.WREADY.next = self.wready.q.val();
        self.axi_bus.BID.next = self.bid.q.val();
        self.axi_bus.BRESP.next = self.bresp.q.val();
        self.axi_bus.BVALID.next = self.bvalid.q.val();
        self.axi_bus.ARREADY.next = self.arready.q.val();

        self.axi_bus.RVALID.next = self.rvalid.q.val();
        self.axi_bus.RDATA.next = self.rdata.q.val();
        self.axi_bus.RID.next = self.rid.q.val();
        self.axi_bus.RRESP.next = self.rresp.q.val();
        self.axi_bus.RLAST.next = self.rlast.q.val();

        // handle a read request
        // Unlike the Xilinx examples, we should have no reason to ever not be able to accept a read address
        if self.axi_bus.ARVALID.val() {
            //self.arready.d.next = true;
            self.axi_rid.d.next = self.axi_bus.ARID.val(); // sample the read ID
            self.axi_raddr.d.next = self.axi_bus.ARADDR.val(); // sample the read address
        } else {
            //self.arready.d.next = false;
        }

        // manually decode the address
        if self.axi_bus.ARADDR.val() == 0 {
            self.read_data.next = self.reg0.q.val();
        } else if self.axi_bus.ARADDR.val() == 4 {
            self.read_data.next = self.reg1.q.val();
        } else if self.axi_bus.ARADDR.val() == 8 {
            self.read_data.next = self.reg2.q.val();
        } else if self.axi_bus.ARADDR.val() == 12 {
            self.read_data.next = self.reg3.q.val();
        } else {
            self.read_data.next = 0.into();
        }

        self.reg_rden.next =
            self.arready.q.val() && self.axi_bus.ARVALID.val() && !self.rvalid.q.val();

        if self.reg_rden.val() {
            self.rid.d.next = self.axi_rid.q.val(); // reflect the read ID
            self.rresp.d.next = 0b00.into(); // OKAY
            self.rdata.d.next = self.read_data.val();
            self.rvalid.d.next = true;
        } else if self.rvalid.q.val() && self.axi_bus.RREADY.val() {
            self.rvalid.d.next = false;
        }

        // By default (not in reset)
        self.awready.d.next = true;
        self.arready.d.next = true;

        // the reset logic: per spec this is asynchronous, but we're going to make it synchronous for now
        // when ARESETn is low:
        if !self.axi_bus.ARESETn.val() {
            self.rvalid.d.next = false;
            self.bvalid.d.next = false;

            self.rresp.d.next = 0b00.into(); // OKAY

            self.awready.d.next = false;
            self.wready.d.next = false;
            self.arready.d.next = false;

            self.bid.d.next = 0b0000.into();
            self.bresp.d.next = 0b00.into();

            self.rid.d.next = 0b0000.into();
            self.rlast.d.next = false;

            self.reg0.d.next = 0.into();
            self.reg1.d.next = 0.into();
            self.reg2.d.next = 0.into();
            self.reg3.d.next = 0.into();

            self.rdata.d.next = 0.into();

            self.axi_raddr.d.next = 0.into();
            self.axi_waddr.d.next = 0.into();
            self.axi_rid.d.next = 0b0000.into();
            self.axi_wid.d.next = 0b0000.into();

            self.reg_rden.next = false;
            self.read_data.next = 0.into();
        }
    }
}
