pub struct State {
    // Data path
    pub pc: u64,
    pub vgpr: [u32; 256],
    pub acc_vgpr: [u32; 256],
    pub sgpr: [32; 104],
    pub lds: [u8; 2 << 18],
    pub exec: u64,
    pub execz: u8,
    pub vcc: u64,
    pub vccz: u8,
    pub scc: u8,

    // Control and status registers
    pub status: u32,
    pub mode: u32,
    pub m0: u32,

    // Trap handling
    pub trapsts: u32,
    pub tba: u64,
    pub tma: u64,
    pub ttmp: [u32; 16],

    // Hardware counters
    pub vmcnt: u8,
    pub expcnt: u8,
    pub lgkmcnt: u8,
}

pub struct StatusReg {

}

fn main() {

}
