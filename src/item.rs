use crate::traits::{Value, Weight};

// Example item struct
#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub weight: usize,
    pub value: usize,
}

impl Weight for Item {
    fn weight(&self) -> usize {
        self.weight
    }
}

impl Value for Item {
    fn value(&self) -> usize {
        self.value
    }
} 