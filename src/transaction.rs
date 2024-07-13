use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Balance {
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub amount: f64,
}

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