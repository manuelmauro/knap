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
    fn to_knapsack_iter(self, capacity: usize) -> crate::optimal::KnapsackIterator<Self::Item> {
        crate::optimal::KnapsackIterator::new(self, capacity)
    }
}

// Implement the extension trait for types satisfying the bounds.
impl<I> KnapsackIterableExt for I
where
    I: IntoIterator + Sized,
    I::Item: Weight + Value + Clone,
{
    // The default implementation is sufficient here.
}

/// An extension trait to easily convert an iterator into a `GreedyKnapsackIterator`.
pub trait ToGreedyKnapsackIterator<T>
where
    Self: IntoIterator<Item = T> + Sized,
    T: Weight + Value + Clone,
{
    /// Converts this iterator into a `GreedyKnapsackIterator` with the given capacity.
    ///
    /// # Arguments
    /// * `capacity`: The maximum capacity of the knapsack for the greedy algorithm.
    ///
    /// # Returns
    /// A `GreedyKnapsackIterator<T>` initialized with the items from this iterator
    /// and the specified capacity.
    fn to_greedy_knapsack_iter(self, capacity: usize) -> crate::greedy::GreedyKnapsackIterator<T> {
        crate::greedy::GreedyKnapsackIterator::new(self, capacity)
    }
}

impl<I, T> ToGreedyKnapsackIterator<T> for I
where
    I: IntoIterator<Item = T> + Sized,
    T: Weight + Value + Clone,
{
    // Use the default implementation from the trait.
}
