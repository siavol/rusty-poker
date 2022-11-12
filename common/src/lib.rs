use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct NewSessionParams {
    pub title: String
}
