/*

This file is an example on how to use the low level api
for disassembly (not reconmmended)

*/

use jacob::codegen::{AssemblyInst, Compilation, FuncAsm, Reg, TargetArch};
use jacob::ir::Module;
use jacob::ir::visibility::Visibilty;
use jacob::x86::regs::{RAX, RDI, RSI};

fn main() {
    let compilation = Compilation {
        funcs: vec![FuncAsm {
            insts: vec![
                AssemblyInst::with3("lea", &RAX.alloc(), &RDI.alloc(), &RSI.alloc()),
                AssemblyInst::with0("ret"),
            ],
            meta_insts: Vec::new(), // We can just leave this empty,
            name: "add".to_owned(),
            scope: Visibilty::Public,
        }],
        arch: TargetArch::X86,
    };

    let module = Module::decompile_comp(compilation);
    for func in &module.funcs {
        println!("{:#?}", func);
    }
}
