use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Session {
    pub title: String,
    pub id: String,
    pub cards: Vec<String>
}
