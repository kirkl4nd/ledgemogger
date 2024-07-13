use uuid::Uuid;

use crate::transaction::{Balance, Transaction};

pub struct Purchase {
    pub id: Uuid,
    pub title: String,
    pub amount: f64,
    pub purchaser: Uuid,
    pub payers: Vec<Uuid>,
    pub purchase_id: Uuid,
}

impl Purchase {
    pub fn to_transaction(&self) -> Transaction {
        let mut transaction = Transaction::new();
        for payer in &self.payers {
            transaction.balance_diffs.push(Balance {
                from_user: *payer,
                to_user: self.purchase_id,
                amount: self.amount / self.payers.len() as f64,
            });
        }

        return transaction;
    }
}