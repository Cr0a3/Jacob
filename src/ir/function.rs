use crate::{
    ir::ir::{IrBlock, IrInst, IrInstDebugNote, IrInstTrait, IrOperand},
    nodes::*,
    ty::{FunctionType, TypeMetadata},
};

/// # Function
///
/// A function in this library saves all relevant ir data for
/// "functions".
/// This includes:
///  - The name
///  - The type
///  - The actual ir
///
/// It is also crucial for building the ir, because it contains
/// the neccessary builder methods
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    pub(crate) name: String,
    pub(crate) function_type: FunctionType,
    pub(crate) ir: Vec<IrBlock>,

    // Used for building the ir
    curr_block_index: usize,
    curr_var_index: usize,
}

impl Function {
    /// Creates a new empty function, with no arguments,
    /// no return type, no ir and no name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            function_type: FunctionType::default(),
            ir: Vec::new(),

            curr_block_index: 0,
            curr_var_index: 0,
        }
    }

    /// Sets the name of the function
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    /// Sets the function type of the function
    pub fn set_function_type(&mut self, fn_ty: FunctionType) {
        self.function_type = fn_ty;
    }

    /// Creates a new ir block with the desired name
    pub fn create_block(&mut self, name: &str) -> usize {
        let id = self.ir.len();
        self.ir.push(IrBlock::Basic {
            name: name.to_owned(),
            insts: Vec::new(),
        });
        id
    }

    /// Selects the given block id as the current one
    pub fn set_block(&mut self, id: usize) {
        if self.ir.len() < id {
            panic!(
                "The id of the block to set as current is outside of the functions avaliable blocks (id: {id} len: {}",
                self.ir.len()
            );
        }

        self.curr_block_index = id;
    }

    /// Requests a new variable from the function
    fn request_new_var(&mut self, name: Option<String>, ty: TypeMetadata) -> IrOperand {
        if let Some(name) = name {
            return IrOperand::Variable { name, ty };
        }

        self.curr_var_index += 1;

        IrOperand::Variable {
            name: self.curr_var_index.to_string(),
            ty,
        }
    }

    /// Gets the current block. If it does not exists it panics
    fn get_block(&mut self) -> &mut IrBlock {
        if self.ir.is_empty() {
            panic!("You need to set an block first, before being able to add ir")
        }

        &mut self.ir[self.curr_block_index]
    }

    /// Checks if both types are equal, else it panics
    fn check_ty_equal(&mut self, ty1: &TypeMetadata, ty2: &TypeMetadata) {
        if *ty1 != *ty2 {
            panic!("The types {ty1:?} and {ty2:?} are not equal");
        }
    }

    /// Adds the given ir instruction to the body of the current ir block
    fn add_block_body(&mut self, inst: Box<dyn IrInstTrait>, dbg: Option<IrInstDebugNote>) {
        match self.get_block() {
            IrBlock::Basic { name: _, insts } => insts.push(IrInst::new(inst, dbg)),
        }
    }

    /// Builds an add ir node
    pub fn add(
        &mut self,
        op1: &IrOperand,
        op2: &IrOperand,
        out_name: Option<String>,
        dbg: Option<IrInstDebugNote>,
    ) -> IrOperand {
        self.check_ty_equal(op1.get_ty(), op2.get_ty());
        let var = self.request_new_var(out_name, *op1.get_ty());

        self.add_block_body(
            IrInstAdd::raw_new(var.clone(), op1.to_owned(), op2.to_owned()),
            dbg,
        );

        var
    }

    /// Builds a sub ir node
    pub fn sub(
        &mut self,
        op1: &IrOperand,
        op2: &IrOperand,
        out_name: Option<String>,
        dbg: Option<IrInstDebugNote>,
    ) -> IrOperand {
        self.check_ty_equal(op1.get_ty(), op2.get_ty());
        let var = self.request_new_var(out_name, *op1.get_ty());

        self.add_block_body(
            IrInstSub::raw_new(var.clone(), op1.to_owned(), op2.to_owned()),
            dbg,
        );

        var
    }

    /// Builds a mul ir node
    pub fn mul(
        &mut self,
        op1: &IrOperand,
        op2: &IrOperand,
        out_name: Option<String>,
        dbg: Option<IrInstDebugNote>,
    ) -> IrOperand {
        self.check_ty_equal(op1.get_ty(), op2.get_ty());
        let var = self.request_new_var(out_name, *op1.get_ty());

        self.add_block_body(
            IrInstMul::raw_new(var.clone(), op1.to_owned(), op2.to_owned()),
            dbg,
        );

        var
    }

    /// Builds a div ir node
    pub fn div(
        &mut self,
        op1: &IrOperand,
        op2: &IrOperand,
        out_name: Option<String>,
        dbg: Option<IrInstDebugNote>,
    ) -> IrOperand {
        self.check_ty_equal(op1.get_ty(), op2.get_ty());
        let var = self.request_new_var(out_name, *op1.get_ty());

        self.add_block_body(
            IrInstDiv::raw_new(var.clone(), op1.to_owned(), op2.to_owned()),
            dbg,
        );

        var
    }

    /// Builds a ret ir node
    pub fn ret(&mut self, op: &IrOperand, dbg: Option<IrInstDebugNote>) {
        self.add_block_body(IrInstRet::raw_new(op.to_owned()), dbg);
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = if let Some(ty) = self.function_type.ret {
            &format!("-> {ty:?}")
        } else {
            ""
        };

        write!(f, "func {}(", self.name)?;
        for (index, (name, ty)) in self.function_type.args.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{name}: {ty:?}")?;
        }
        writeln!(f, ") {ret} {{")?;

        for block in &self.ir {
            match block {
                IrBlock::Basic { name, insts } => {
                    writeln!(f, "  .b{name}")?;

                    for inst in insts {
                        writeln!(f, "    {}", inst.dump())?;
                    }
                }
            }
        }

        writeln!(f, "}}")
    }
}
