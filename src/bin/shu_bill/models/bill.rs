use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Bill {
    pub(crate) id: i32,
    pub(crate) debtor_name: String,
    pub(crate) reason: String,
    pub(crate) amount_owed: f64,
}

impl Bill {
    pub(crate) fn display(&self) {
        println!("ID: {}", self.id);
        println!("Debtor: {}", self.debtor_name);
        println!("Amount owned: {}", self.amount_owed);
        println!("Reason: {}", self.reason);
    }
}
