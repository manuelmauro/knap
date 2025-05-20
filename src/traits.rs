/// Defines the behavior for items that have a weight.
///
/// This trait is used to abstract the concept of weight for items
/// that can be placed in a knapsack.
pub trait Weight {
    /// Returns the weight of the item.
    ///
    /// The weight must be a non-negative integer.
    fn weight(&self) -> usize;
}

/// Defines the behavior for items that have a value.
///
/// This trait is used to abstract the concept of value for items
/// that can be placed in a knapsack.
pub trait Value {
    /// Returns the value of the item.
    ///
    /// The value must be a non-negative integer.
    fn value(&self) -> usize;
}

/// An extension trait to easily convert an iterator into a `KnapsackIterator`.
///
/// This trait provides a convenient way to create an optimal knapsack solver
/// directly from an iterator of items that implement `Weight`, `Value`, and `Clone`.
pub trait ToKnapsackIterator: IntoIterator + Sized
where
    Self::Item: Weight + Value + Clone,
{
    /// Converts this iterator into a `KnapsackIterator` with the given capacity.
    ///
    /// This method is used to initialize the optimal knapsack algorithm.
    ///
    /// # Arguments
    ///
    /// * `capacity`: The maximum capacity of the knapsack.
    ///
    /// # Returns
    ///
    /// A `KnapsackIterator<Self::Item>` initialized with the items from this iterator
    /// and the specified capacity, ready to compute the optimal solution.
    fn to_knapsack_iter(self, capacity: usize) -> crate::optimal::KnapsackIterator<Self::Item> {
        crate::optimal::KnapsackIterator::new(self, capacity)
    }
}

// Blanket implementation of `ToKnapsackIterator` for any type that meets the bounds.
impl<I> ToKnapsackIterator for I
where
    I: IntoIterator + Sized,
    I::Item: Weight + Value + Clone,
{
    // The default implementation provided by the trait is used.
}

/// An extension trait to easily convert an iterator into a `GreedyKnapsackIterator`.
///
/// This trait provides a convenient way to create a greedy knapsack solver
/// directly from an iterator of items that implement `Weight`, `Value`, and `Clone`.
pub trait ToGreedyKnapsackIterator<T>
where
    Self: IntoIterator<Item = T> + Sized,
    T: Weight + Value + Clone,
{
    /// Converts this iterator into a `GreedyKnapsackIterator` with the given capacity.
    ///
    /// This method is used to initialize the greedy knapsack algorithm.
    ///
    /// # Arguments
    ///
    /// * `capacity`: The maximum capacity of the knapsack for the greedy algorithm.
    ///
    /// # Returns
    ///
    /// A `GreedyKnapsackIterator<T>` initialized with the items from this iterator
    /// and the specified capacity, ready to provide a greedy solution.
    fn to_greedy_knapsack_iter(self, capacity: usize) -> crate::greedy::GreedyKnapsackIterator<T> {
        crate::greedy::GreedyKnapsackIterator::new(self, capacity)
    }
}

// Blanket implementation of `ToGreedyKnapsackIterator` for any type that meets the bounds.
impl<I, T> ToGreedyKnapsackIterator<T> for I
where
    I: IntoIterator<Item = T> + Sized,
    T: Weight + Value + Clone,
{
    // The default implementation provided by the trait is used.
}
