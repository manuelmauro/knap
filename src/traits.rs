// Trait for items that have a weight
pub trait Weight {
    fn weight(&self) -> usize;
}

// Trait for items that have a value
pub trait Value {
    fn value(&self) -> usize;
}

// Extension trait to provide a convenient method for creating a KnapsackIterator
pub trait KnapsackIterableExt: IntoIterator + Sized
where
    Self::Item: Weight + Value + Clone,
{
    fn to_knapsack_iter(self, capacity: usize) -> crate::knapsack::KnapsackIterator<Self::Item> {
        crate::knapsack::KnapsackIterator::new(self, capacity)
    }
}

// Implement the extension trait for all types that satisfy the bounds.
impl<I> KnapsackIterableExt for I
where
    I: IntoIterator + Sized,
    I::Item: Weight + Value + Clone,
{
    // The default implementation is sufficient here.
} 