use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Record {
    pub id: i32,
    pub name: String,
}
