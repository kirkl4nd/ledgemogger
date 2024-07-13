use crate::transaction::Balance;
use uuid::Uuid;

use crate::transaction::Transaction;

pub struct PaymentInput {
    pub amount: f64,
    pub payer: Uuid,
    pub payee: Uuid,
}

impl PaymentInput {

    pub fn to_payment(self) -> Payment {
        Payment {
            id: Uuid::new_v4(),
            amount: self.amount,
            payer: self.payer,
            payee: self.payee,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Payment {
    pub id: Uuid,
    pub amount: f64,
    pub payer: Uuid,
    pub payee: Uuid,
}

impl Payment {
    pub fn to_transaction(&self) -> Transaction {
        let mut transaction = Transaction::new();
        transaction.balance_diffs.push(Balance {
            from_user: self.payer,
            to_user: self.payee,
            amount: self.amount,
        });
        transaction
    }
}
