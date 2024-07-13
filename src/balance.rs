use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Balance {
    pub from_user: Uuid,
    pub to_user: Uuid,
    pub amount: f64,
}