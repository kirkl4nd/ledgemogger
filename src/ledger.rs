use std::error::Error;

use uuid::Uuid;

use crate::{payment::{Payment, PaymentInput}, purchase::{Purchase, PurchaseInput}, transaction::Transaction, user::{User, UserInput}};

#[derive(Debug)]
pub struct Ledger {
    pub users: Vec<User>,
    pub purchases: Vec<Purchase>,
    pub payments: Vec<Payment>,
    pub transactions: Vec<Transaction>,
}

impl Ledger {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            purchases: Vec::new(),
            payments: Vec::new(),
            transactions: Vec::new(),
        }
    }

    /// ### Adding and looking up users, and user id validation

    pub fn add_user(&mut self, user_input: UserInput) -> Result<(), Box<dyn Error>> {
        // if username already exists, return an error
        if self.users.iter().any(|u| u.username == user_input.username) {
            return Err("Username already exists".into());
        }

        let user = User {
            id: Uuid::new_v4(),
            username: user_input.username,
            name: user_input.name,
        };

        self.users.push(user);
        Ok(())
    }

    pub fn get_users(&self) -> Vec<User> {
        self.users.clone()
    }

    pub fn lookup_user_by_id(&self, id: Uuid) -> Option<User> {
        self.users.iter().find(|u| u.id == id).cloned()
    }

    pub fn lookup_user_by_username(&self, username: &str) -> Option<User> {
        self.users.iter().find(|u| u.username == username).cloned()
    }

    pub fn validate_user_id(&self, user_id: Uuid) -> bool {
        self.users.iter().any(|u| u.id == user_id)
    }

    /// ### Adding and looking up purchases and transactions
    
    pub fn add_purchase(&mut self, purchase_input: PurchaseInput) {
        // Convert purchase input to purchase
        let purchase = purchase_input.to_purchase();
        
        // Create transaction from purchase
        let transaction = purchase.to_transaction();
        
        // Add purchase to self.purchases
        self.purchases.push(purchase);
        
        // Add transaction to self.transactions
        self.transactions.push(transaction);
    }

    pub fn add_payment(&mut self, payment_input: PaymentInput) {
        // Convert payment input to payment
        let payment = payment_input.to_payment();
        
        // Create transaction from payment
        let transaction = payment.to_transaction();
        
        // Add payment to self.payments
        self.payments.push(payment);
        
        // Add transaction to self.transactions
        self.transactions.push(transaction);
    }

    pub fn get_purchases(&self) -> Vec<Purchase> {
        self.purchases.clone()
    }

    pub fn get_payments(&self) -> Vec<Payment> {
        self.payments.clone()
    }

    pub fn get_transactions(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn lookup_purchase_by_id(&self, id: Uuid) -> Option<Purchase> {
        self.purchases.iter().find(|p| p.id == id).cloned()
    }

    pub fn lookup_payment_by_id(&self, id: Uuid) -> Option<Payment> {
        self.payments.iter().find(|p| p.id == id).cloned()
    }

    pub fn lookup_transaction_by_id(&self, id: Uuid) -> Option<Transaction> {
        self.transactions.iter().find(|t| t.id == id).cloned()
    }

}