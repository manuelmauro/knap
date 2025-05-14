// Trait for items that have a weight
pub trait Weight {
    fn weight(&self) -> usize;
}

// Trait for items that have a value
pub trait Value {
    fn value(&self) -> usize;
} 