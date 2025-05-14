// Trait for item weight.
pub trait Weight {
    fn weight(&self) -> usize;
}

// Trait for item value.
pub trait Value {
    fn value(&self) -> usize;
}

// Extension trait for creating a KnapsackIterator.
pub trait KnapsackIterableExt: IntoIterator + Sized
where
    Self::Item: Weight + Value + Clone,
{
    fn to_knapsack_iter(self, capacity: usize) -> crate::knapsack::KnapsackIterator<Self::Item> {
        crate::knapsack::KnapsackIterator::new(self, capacity)
    }
}

// Implement the extension trait for types satisfying the bounds.
impl<I> KnapsackIterableExt for I
where
    I: IntoIterator + Sized,
    I::Item: Weight + Value + Clone,
{
    // Default implementation is sufficient.
} 