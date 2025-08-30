use std::collections::{HashMap, HashSet};

use crate::{ir::Function, opt::Optimization};

/// Dead code elimination
pub struct Dce {}

impl Optimization for Dce {
    fn name(&self) -> &'static str {
        "Dead Code Elimination"
    }

    fn run(&self, func: &mut Function) {
        let mut live: HashSet<u64> = HashSet::new();
        let mut users: HashMap<u64, u64> = HashMap::new();

        for op in func.ir.iter().rev() {
            let node = op.force_node().borrow();
            if node.is_ret() {
                live.insert(node.hash_u64());
            }

            for user in node.ops.iter() {
                if user.is_out() {
                    *users
                        .entry(user.force_node().borrow().hash_u64())
                        .or_insert(0) += 1;
                }
            }
        }

        let mut to_delete = Vec::new();
        for (id, users) in users {
            if users <= 1 {
                to_delete.push(id);
            }
        }

        func.ir.retain(|op| {
            live.contains(&op.force_node().borrow().hash_u64())
                && !to_delete.contains(&op.force_node().borrow().hash_u64())
        });
    }
}
