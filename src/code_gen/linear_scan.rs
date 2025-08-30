use std::collections::HashMap;

use crate::{
    code_gen::target::Target, mc::Allocation, mcinst_builder::McInstBuilder, ty::TypeMetadata,
};

/// The linear scan register allocator
///
/// Implements: https://anoopsarkar.github.io/compilers-class/assets/lectures/opt3-regalloc-linearscan.pdf
pub struct LinearScan<'b> {
    builder: &'b mut McInstBuilder,
}

impl<'b> LinearScan<'b> {
    /// Creates a new linear scan register allocator
    pub fn new(builder: &'b mut McInstBuilder, target: Target) -> Self {
        Self { builder }
    }

    fn allocate_register(
        _used: usize,
        ty: TypeMetadata,
        gpr_regs: &mut Vec<usize>,
        last_spill_adr: &mut usize,
        freed_spill_locs: &mut Vec<usize>,
    ) -> Allocation {
        // ToDo: use the `used` argument to rank the variable to see if they are variables which are more frequently used
        // so that they'll get "vip" access

        if let Some(reg) = gpr_regs.pop() {
            return Allocation::Register { num: reg, ty };
        }

        let mut pos = *last_spill_adr;
        if let Some(freed_pos) = freed_spill_locs.pop() {
            pos = freed_pos;
        } else {
            *last_spill_adr += 1;
        }

        Allocation::Spill { pos, ty }
    }

    fn free_register(
        resource: &Allocation,
        gpr_regs: &mut Vec<usize>,
        freed_spill_locs: &mut Vec<usize>,
    ) {
        match resource {
            Allocation::Register { num, ty: _ } => gpr_regs.push(*num),
            Allocation::Spill { pos, ty: _ } => freed_spill_locs.push(*pos),
        }
    }

    /// Runs the register allocator
    pub fn run(&mut self) {
        for func in self.builder.funcs.iter_mut() {
            let mut curr_tick = 0;
            let mut used_counter: HashMap<usize, usize> = HashMap::new();
            let mut first_enconter: HashMap<usize, usize> = HashMap::new();
            let mut last_enconter: HashMap<usize, usize> = HashMap::new();

            let mut gpr_regs: Vec<usize> = Vec::new();
            let mut freed_spill_locs: Vec<usize> = Vec::new();
            let mut last_spill_adr: usize = 0;
            let mut uses_stack: bool = false;

            for block in func.blocks.iter().rev() {
                for node in block.nodes.iter().rev() {
                    for op in &node.borrow().operands {
                        let id = op.node.borrow().node_id;

                        last_enconter.entry(id).or_insert(curr_tick);
                        *first_enconter.entry(id).or_insert(curr_tick) = curr_tick;
                        *used_counter.entry(id).or_insert(0) += 1;
                    }

                    curr_tick += 1;
                }
            }

            // We now have the list of the most used variables and their timeframes.
            // Note: this is purly based on how many nodes use the given instruction and not,
            // if the instruction is in a loop

            for block in &mut func.blocks {
                for tick in 0..curr_tick {
                    for node in &mut block.nodes {
                        if !node.borrow().has_out() {
                            continue;
                        }

                        let id = node.borrow().node_id;
                        let first_seen = *first_enconter.get(&id).unwrap() as isize;

                        if tick < (first_seen - (curr_tick as isize * 2)).unsigned_abs() {
                            // the encounter method is bottom up and this loop is top down, cuz of this we need to recalc the first encounter tick
                            continue;
                        }

                        let last_seen = *last_enconter.get(&id).unwrap() as isize;
                        let used = *used_counter.get(&id).unwrap();

                        let out = &mut node.borrow_mut().out_allocate;
                        if out.is_none() {
                            let alloc = LinearScan::allocate_register(
                                used,
                                node.borrow().value_types[0],
                                &mut gpr_regs,
                                &mut last_spill_adr,
                                &mut freed_spill_locs,
                            );

                            uses_stack |= alloc.is_mem();
                            *out = Some(alloc);
                        } else if let Some(resource) = out
                            && tick > (last_seen - (curr_tick as isize * 2)).unsigned_abs()
                        {
                            LinearScan::free_register(
                                resource,
                                &mut gpr_regs,
                                &mut freed_spill_locs,
                            );
                        }
                    }
                }
            }
        }
    }
}
