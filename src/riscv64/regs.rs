use crate::codegen::Reg;

/// A register for the riscv64 platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Riscv64Reg {
    id: usize,
}

/// The a0 register
pub const A0: Riscv64Reg = Riscv64Reg { id: 0 };
pub(crate) const A0_ID: usize = 0;
/// The a1 register
pub const A1: Riscv64Reg = Riscv64Reg { id: 1 };
pub(crate) const A1_ID: usize = 1;
/// The a2 register
pub const A2: Riscv64Reg = Riscv64Reg { id: 2 };
pub(crate) const A2_ID: usize = 2;
/// The a3 register
pub const A3: Riscv64Reg = Riscv64Reg { id: 3 };
pub(crate) const A3_ID: usize = 3;
/// The a4 register
pub const A4: Riscv64Reg = Riscv64Reg { id: 4 };
pub(crate) const A4_ID: usize = 4;
/// The a5 register
pub const A5: Riscv64Reg = Riscv64Reg { id: 5 };
pub(crate) const A5_ID: usize = 5;
/// The a6 register
pub const A6: Riscv64Reg = Riscv64Reg { id: 6 };
pub(crate) const A6_ID: usize = 6;
/// The a7 register
pub const A7: Riscv64Reg = Riscv64Reg { id: 7 };
pub(crate) const A7_ID: usize = 7;
/// The s2 register
pub const S2: Riscv64Reg = Riscv64Reg { id: 8 };
pub(crate) const S2_ID: usize = 8;
/// The s3 register
pub const S3: Riscv64Reg = Riscv64Reg { id: 9 };
pub(crate) const S3_ID: usize = 9;
/// The s4 register
pub const S4: Riscv64Reg = Riscv64Reg { id: 10 };
pub(crate) const S4_ID: usize = 10;
/// The s5 register
pub const S5: Riscv64Reg = Riscv64Reg { id: 11 };
pub(crate) const S5_ID: usize = 11;
/// The s6 register
pub const S6: Riscv64Reg = Riscv64Reg { id: 12 };
pub(crate) const S6_ID: usize = 12;
/// The s7 register
pub const S7: Riscv64Reg = Riscv64Reg { id: 13 };
pub(crate) const S7_ID: usize = 13;
/// The s8 register
pub const S8: Riscv64Reg = Riscv64Reg { id: 14 };
pub(crate) const S8_ID: usize = 14;
/// The s9 register
pub const S9: Riscv64Reg = Riscv64Reg { id: 15 };
pub(crate) const S9_ID: usize = 15;
/// The s10 register
pub const S10: Riscv64Reg = Riscv64Reg { id: 16 };
pub(crate) const S10_ID: usize = 16;
/// The s11 register
pub const S11: Riscv64Reg = Riscv64Reg { id: 17 };
pub(crate) const S11_ID: usize = 17;
/// The t1 register
pub const T1: Riscv64Reg = Riscv64Reg { id: 18 };
pub(crate) const T1_ID: usize = 18;
/// The t2 register
pub const T2: Riscv64Reg = Riscv64Reg { id: 19 };
pub(crate) const T2_ID: usize = 19;
/// The t3 register
pub const T3: Riscv64Reg = Riscv64Reg { id: 20 };
pub(crate) const T3_ID: usize = 20;
/// The t4 register
pub const T4: Riscv64Reg = Riscv64Reg { id: 21 };
pub(crate) const T4_ID: usize = 21;
/// The t5 register
pub const T5: Riscv64Reg = Riscv64Reg { id: 22 };
pub(crate) const T5_ID: usize = 22;
/// The t6 register
pub const T6: Riscv64Reg = Riscv64Reg { id: 23 };
pub(crate) const T6_ID: usize = 23;
/// The t7 register
pub const T7: Riscv64Reg = Riscv64Reg { id: 24 };
pub(crate) const T7_ID: usize = 24;

impl Reg for Riscv64Reg {
    fn id(&self) -> usize {
        self.id
    }

    fn ty(&self) -> crate::ir::TypeMetadata {
        crate::ir::TypeMetadata::Int64
    }

    fn name(&self) -> &'static str {
        match self.id {
            val if val == A0.id() => "a0",
            val if val == A1.id() => "a1",
            val if val == A2.id() => "a2",
            val if val == A3.id() => "a3",
            val if val == A4.id() => "a4",
            val if val == A5.id() => "a5",
            val if val == A6.id() => "a6",
            val if val == A7.id() => "a7",
            val if val == S2.id() => "s2",
            val if val == S3.id() => "s3",
            val if val == S4.id() => "s4",
            val if val == S5.id() => "s5",
            val if val == S6.id() => "s6",
            val if val == S7.id() => "s7",
            val if val == S8.id() => "s8",
            val if val == S9.id() => "s9",
            val if val == S10.id() => "s10",
            val if val == S11.id() => "s11",
            val if val == T1.id() => "t1",
            val if val == T2.id() => "t2",
            val if val == T3.id() => "t3",
            val if val == T4.id() => "t4",
            val if val == T5.id() => "t5",
            val if val == T6.id() => "t6",
            val if val == T7.id() => "t7",
            _ => panic!(),
        }
    }

    fn is_gpr(&self) -> bool {
        true
    }

    fn caller_saved(&self) -> bool {
        !(S2_ID..S11_ID).contains(&self.id)
    }
}
