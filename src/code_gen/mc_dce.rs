use std::collections::{HashMap, HashSet};

use crate::mcinst_builder::McInstBuilder;

/// This function removes unused instructions
pub fn mcinst_dce(builder: &mut McInstBuilder) {
    for func in &mut builder.funcs {
        let mut live: HashSet<usize> = HashSet::new();
        let mut users: HashMap<usize, usize> = HashMap::new();

        for block in &func.blocks {
            for node in block.nodes.iter().rev() {
                if node.borrow().crucial() {
                    live.insert(node.borrow().node_id);
                }

                for user in node.borrow().operands.iter() {
                    *users.entry(user.node.borrow().node_id).or_insert(0) += 1;
                }
            }
        }

        let mut to_delete = Vec::new();
        for (id, users) in users {
            if users <= 1 {
                to_delete.push(id);
            }
        }

        for block in &mut func.blocks {
            block.nodes.retain(|node| {
                live.contains(&node.borrow().node_id) && !to_delete.contains(&node.borrow().node_id)
            });
        }
    }
}
