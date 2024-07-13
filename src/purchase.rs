use uuid::Uuid;

use crate::transaction::{Balance, Transaction};

pub struct PurchaseInput {
    pub title: String,
    pub amount: f64,
    pub purchaser: Uuid,
    pub payers: Vec<Uuid>,
}

impl PurchaseInput {

    pub fn to_purchase(self) -> Purchase {
        Purchase {
            id: Uuid::new_v4(),
            title: self.title,
            amount: self.amount,
            purchaser: self.purchaser,
            payers: self.payers,
            purchase_id: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone)]
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

        let amount: f64 = self.amount / self.payers.len() as f64;

        let mut transaction = Transaction::new();
        for payer in &self.payers {
            transaction.balance_diffs.push(Balance {
                from_user: *payer,
                to_user: self.purchaser,
                amount: -amount,
            });
        }

        return transaction;
    }
}