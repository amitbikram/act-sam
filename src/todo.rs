use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}


impl Todo {
    pub fn new(id: i32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoRequest {
    pub title: String,
    pub completed: bool,
}
