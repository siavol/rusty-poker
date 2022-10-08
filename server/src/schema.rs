use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct NewSessionParams {
    pub title: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Session {
    pub title: String,
    pub id: String,
    pub cards: Vec<String>
}
