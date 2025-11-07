use crate::codegen::Reg;

/// A register for the aarch64 platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aarch64Reg {
    id: usize,
}

/// The x0 register
pub const X0: Aarch64Reg = Aarch64Reg { id: 0 };
pub(crate) const X0_ID: usize = 0;
/// The x1 register
pub const X1: Aarch64Reg = Aarch64Reg { id: 1 };
pub(crate) const X1_ID: usize = 1;
/// The x2 register
pub const X2: Aarch64Reg = Aarch64Reg { id: 2 };
pub(crate) const X2_ID: usize = 2;
/// The x3 register
pub const X3: Aarch64Reg = Aarch64Reg { id: 3 };
pub(crate) const X3_ID: usize = 3;
/// The x4 register
pub const X4: Aarch64Reg = Aarch64Reg { id: 4 };
pub(crate) const X4_ID: usize = 4;
/// The x5 register
pub const X5: Aarch64Reg = Aarch64Reg { id: 5 };
pub(crate) const X5_ID: usize = 5;
/// The x6 register
pub const X6: Aarch64Reg = Aarch64Reg { id: 6 };
pub(crate) const X6_ID: usize = 6;
/// The x7 register
pub const X7: Aarch64Reg = Aarch64Reg { id: 7 };
pub(crate) const X7_ID: usize = 7;
/// The x8 register
pub const X8: Aarch64Reg = Aarch64Reg { id: 8 };
pub(crate) const X8_ID: usize = 8;
/// The x9 register
pub const X9: Aarch64Reg = Aarch64Reg { id: 9 };
pub(crate) const X9_ID: usize = 9;
/// The x10 register
pub const X10: Aarch64Reg = Aarch64Reg { id: 10 };
pub(crate) const X10_ID: usize = 10;
/// The x11 register
pub const X11: Aarch64Reg = Aarch64Reg { id: 11 };
pub(crate) const X11_ID: usize = 11;
/// The x12 register
pub const X12: Aarch64Reg = Aarch64Reg { id: 12 };
pub(crate) const X12_ID: usize = 12;
/// The x13 register
pub const X13: Aarch64Reg = Aarch64Reg { id: 13 };
pub(crate) const X13_ID: usize = 13;
/// The x14 register
pub const X14: Aarch64Reg = Aarch64Reg { id: 14 };
pub(crate) const X14_ID: usize = 14;
/// The x15 register
pub const X15: Aarch64Reg = Aarch64Reg { id: 15 };
pub(crate) const X15_ID: usize = 15;
/// The x16 register
pub const X16: Aarch64Reg = Aarch64Reg { id: 16 };
pub(crate) const X16_ID: usize = 16;
/// The x17 register
pub const X17: Aarch64Reg = Aarch64Reg { id: 17 };
pub(crate) const X17_ID: usize = 17;
/// The x18 register
pub const X18: Aarch64Reg = Aarch64Reg { id: 18 };
pub(crate) const X18_ID: usize = 18;
/// The x19 register
pub const X19: Aarch64Reg = Aarch64Reg { id: 19 };
pub(crate) const X19_ID: usize = 19;
/// The x20 register
pub const X20: Aarch64Reg = Aarch64Reg { id: 20 };
pub(crate) const X20_ID: usize = 20;
/// The x21 register
pub const X21: Aarch64Reg = Aarch64Reg { id: 21 };
pub(crate) const X21_ID: usize = 21;
/// The x22 register
pub const X22: Aarch64Reg = Aarch64Reg { id: 22 };
pub(crate) const X22_ID: usize = 22;
/// The x23 register
pub const X23: Aarch64Reg = Aarch64Reg { id: 23 };
pub(crate) const X23_ID: usize = 23;
/// The x24 register
pub const X24: Aarch64Reg = Aarch64Reg { id: 24 };
pub(crate) const X24_ID: usize = 24;
/// The x25 register
pub const X25: Aarch64Reg = Aarch64Reg { id: 25 };
pub(crate) const X25_ID: usize = 25;
/// The x26 register
pub const X26: Aarch64Reg = Aarch64Reg { id: 26 };
pub(crate) const X26_ID: usize = 26;
/// The x27 register
pub const X27: Aarch64Reg = Aarch64Reg { id: 27 };
pub(crate) const X27_ID: usize = 27;
/// The x28 register
pub const X28: Aarch64Reg = Aarch64Reg { id: 28 };
pub(crate) const X28_ID: usize = 28;
/// The sp register
pub const SP: Aarch64Reg = Aarch64Reg { id: 29 };
pub(crate) const SP_ID: usize = 29;

impl Reg for Aarch64Reg {
    fn id(&self) -> usize {
        self.id
    }

    fn ty(&self) -> crate::ir::TypeMetadata {
        crate::ir::TypeMetadata::Int64
    }

    fn name(&self) -> &'static str {
        match self.id {
            val if val == X0.id() => "x0",
            val if val == X1.id() => "x1",
            val if val == X2.id() => "x2",
            val if val == X3.id() => "x3",
            val if val == X4.id() => "x4",
            val if val == X5.id() => "x5",
            val if val == X6.id() => "x6",
            val if val == X7.id() => "x7",
            val if val == X8.id() => "x8",
            val if val == X9.id() => "x9",
            val if val == X10.id() => "x10",
            val if val == X11.id() => "x11",
            val if val == X12.id() => "x12",
            val if val == X13.id() => "x13",
            val if val == X14.id() => "x14",
            val if val == X15.id() => "x15",
            val if val == X16.id() => "x16",
            val if val == X17.id() => "x17",
            val if val == X18.id() => "x18",
            val if val == X19.id() => "x19",
            val if val == X20.id() => "x20",
            val if val == X21.id() => "x21",
            val if val == X22.id() => "x22",
            val if val == X23.id() => "x23",
            val if val == X24.id() => "x24",
            val if val == X25.id() => "x25",
            val if val == X26.id() => "x26",
            val if val == X27.id() => "x27",
            val if val == X28.id() => "x28",
            val if val == SP.id() => "sp",
            _ => panic!(),
        }
    }

    fn is_gpr(&self) -> bool {
        !matches!(self.id, SP_ID)
    }

    fn caller_saved(&self) -> bool {
        self.id <= 7 && self.id != SP_ID
    }
}
