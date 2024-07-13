use uuid::Uuid;

use crate::balance::Balance;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub balance_diffs: Vec<Balance>,
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            id: Uuid::new_v4(),
            balance_diffs: Vec::new(),
        }
    }
}