use ledger::Ledger;
use purchase::{Purchase, PurchaseInput};
use user::UserInput;

mod ledger;
mod payment;
mod purchase;
mod transaction;
mod user;

fn main() {
    let mut ledger: Ledger = Ledger::new();
    let user_input = UserInput::new("kaburrill".to_string(), "Kirk Burrill".to_string());
    ledger.add_user(user_input).unwrap();

    let user_input = UserInput::new("tdubay".to_string(), "Tyler Dubay".to_string());
    ledger.add_user(user_input).unwrap();

    let user_input = UserInput::new("mhinrichs".to_string(), "Michael Hinrichs".to_string());
    ledger.add_user(user_input).unwrap();

    let kaburrill_id = ledger.lookup_user_by_username("kaburrill").unwrap().id;
    let tdubay_id = ledger.lookup_user_by_username("tdubay").unwrap().id;
    let mhinrichs_id = ledger.lookup_user_by_username("mhinrichs").unwrap().id;

    let purchase_input_1 = PurchaseInput {
        title: "Pots and pans".to_owned(),
        amount: 149.99,
        purchaser: mhinrichs_id,
        payers: vec![kaburrill_id, tdubay_id, mhinrichs_id],
    };

    ledger.add_purchase(purchase_input_1);

    println!("{:?}", ledger);

}
