use std::collections::HashMap;
use uuid::Uuid;

use crate::{balance::Balance, ledger::Ledger};

impl Ledger {
    pub fn simplify_balances(&self) -> Vec<Balance> {
        let mut net_balances: HashMap<Uuid, f64> = HashMap::new();

        // Calculate net balances for each user
        for transaction in &self.transactions {
            for balance in &transaction.balance_diffs {
                *net_balances.entry(balance.from_user).or_insert(0.0) += balance.amount;
                *net_balances.entry(balance.to_user).or_insert(0.0) -= balance.amount;
            }
        }

        // Separate users into creditors and debtors
        let mut creditors: Vec<(Uuid, f64)> = net_balances.iter()
            .filter(|&(_, &amount)| amount > 0.0)
            .map(|(&user, &amount)| (user, amount))
            .collect();

        let mut debtors: Vec<(Uuid, f64)> = net_balances.iter()
            .filter(|&(_, &amount)| amount < 0.0)
            .map(|(&user, &amount)| (user, -amount))
            .collect();

        let mut simplified_balances = Vec::new();

        // Resolve balances
        while !creditors.is_empty() && !debtors.is_empty() {
            let (creditor, credit_amount) = creditors.pop().unwrap();
            let (debtor, debt_amount) = debtors.pop().unwrap();

            let amount = credit_amount.min(debt_amount);

            simplified_balances.push(Balance {
                from_user: debtor,
                to_user: creditor,
                amount,
            });

            if credit_amount > debt_amount {
                creditors.push((creditor, credit_amount - amount));
            } else if debt_amount > credit_amount {
                debtors.push((debtor, debt_amount - amount));
            }
        }

        simplified_balances
    }

    pub fn display_balances(&self, balances: Vec<Balance>) {
        for balance in balances {
            let from_user = self.users.iter().find(|user| user.id == balance.from_user).unwrap();
            let to_user = self.users.iter().find(|user| user.id == balance.to_user).unwrap();
            println!("{} owes {} ${:.2}", from_user.name, to_user.name, balance.amount);
        }
    }
}