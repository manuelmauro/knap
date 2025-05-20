use crate::traits::{Value, Weight};

/// An iterator that yields items based on a greedy approximation
/// for the knapsack problem.
///
/// This iterator computes the greedy solution upon creation and then
/// yields the selected items one by one.
///
/// # Examples
///
/// ```
/// use knap::traits::{Value, Weight};
/// use knap::greedy::GreedyKnapsackIterator;
///
/// #[derive(Clone, Debug, PartialEq)]
/// struct Item {
///     id: usize,
///     weight: usize,
///     value: usize,
/// }
///
/// impl Weight for Item {
///     fn weight(&self) -> usize {
///         self.weight
///     }
/// }
///
/// impl Value for Item {
///     fn value(&self) -> usize {
///         self.value
///     }
/// }
///
/// let items = vec![
///     Item { id: 1, weight: 2, value: 10 },
///     Item { id: 2, weight: 3, value: 12 },
///     Item { id: 3, weight: 1, value: 8 },
/// ];
/// let capacity = 4;
///
/// let mut greedy_iter = GreedyKnapsackIterator::new(items.clone(), capacity);
///
/// let mut selected_items = Vec::new();
/// while let Some(item) = greedy_iter.next() {
///    selected_items.push(item);
/// }
///
/// assert_eq!(selected_items, vec![
///     Item { id: 3, weight: 1, value: 8 }, // Highest ratio (8/1 = 8)
///     Item { id: 1, weight: 2, value: 10 }, // Next highest ratio (10/2 = 5)
///                                          // Item 2 (12/3 = 4) doesn't fit
/// ]);
/// ```
#[derive(Debug)]
pub struct GreedyKnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    solution_items: Vec<T>,
    current_index: usize,
}

impl<T> GreedyKnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    /// Computes an approximate solution using a greedy algorithm.
    ///
    /// Items are sorted by their value-to-weight ratio in descending order,
    /// and items are picked as long as they fit. Items with zero weight
    /// and positive value are prioritized.
    fn calculate_greedy_items(items_list: &[T], capacity_val: usize) -> Vec<T> {
        if items_list.is_empty() || capacity_val == 0 {
            return Vec::new();
        }

        let mut items_with_meta: Vec<(usize, f64, usize)> = items_list
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let weight = item.weight();
                let value = item.value();
                let ratio = if weight > 0 {
                    value as f64 / weight as f64
                } else if value > 0 {
                    f64::MAX
                } else {
                    -1.0
                };
                (idx, ratio, value)
            })
            .collect();

        items_with_meta.sort_by(|a, b| {
            let ratio_cmp = b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal);
            if ratio_cmp == std::cmp::Ordering::Equal {
                // If ratios are equal (e.g., both are MAX for zero-weight items),
                // prioritize by value for zero-weight items, or by original index for stability otherwise.
                if a.1 == f64::MAX && b.1 == f64::MAX {
                    b.2.cmp(&a.2) // Higher value first for zero-weight items
                } else {
                    a.0.cmp(&b.0) // Stable sort for other items with same ratio
                }
            } else {
                ratio_cmp
            }
        });

        let mut result_items = Vec::new();
        let mut current_capacity = capacity_val;

        for (original_idx, _ratio, _value) in items_with_meta {
            let item = &items_list[original_idx];
            let item_weight = item.weight();

            if item_weight <= current_capacity {
                result_items.push(item.clone());
                current_capacity -= item_weight;
            }
        }
        result_items
    }

    /// Creates a new `GreedyKnapsackIterator`.
    ///
    /// It computes the greedy solution for the given items and capacity
    /// and prepares to iterate over this solution.
    ///
    /// # Arguments
    ///
    /// * `input_items`: An iterator over items that implement `Weight`, `Value`, and `Clone`.
    /// * `capacity`: The maximum capacity of the knapsack.
    ///
    /// # Returns
    ///
    /// A new `GreedyKnapsackIterator` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use knap::traits::{Value, Weight};
    /// use knap::greedy::GreedyKnapsackIterator;
    ///
    /// #[derive(Clone, Debug, PartialEq)]
    /// struct Item {
    ///     id: usize,
    ///     weight: usize,
    ///     value: usize,
    /// }
    ///
    /// impl Weight for Item {
    ///     fn weight(&self) -> usize {
    ///         self.weight
    ///     }
    /// }
    ///
    /// impl Value for Item {
    ///     fn value(&self) -> usize {
    ///         self.value
    ///     }
    /// }
    ///
    /// let items = vec![
    ///     Item { id: 1, weight: 2, value: 10 },
    ///     Item { id: 2, weight: 3, value: 12 },
    /// ];
    /// let capacity = 3;
    /// let greedy_iter = GreedyKnapsackIterator::new(items, capacity);
    /// // The iterator is now ready to yield items from the greedy solution.
    /// ```
    pub fn new(input_items: impl IntoIterator<Item = T>, capacity: usize) -> Self {
        let items_vec: Vec<T> = input_items.into_iter().collect();
        let solution_items = Self::calculate_greedy_items(&items_vec, capacity);

        GreedyKnapsackIterator {
            solution_items,
            current_index: 0,
        }
    }
}

impl<T> Iterator for GreedyKnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    type Item = T;

    /// Advances the iterator and returns the next item from the greedy solution.
    ///
    /// Returns `None` when the iteration is finished.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.solution_items.len() {
            let item = self.solution_items[self.current_index].clone();
            self.current_index += 1;
            Some(item)
        } else {
            None
        }
    }
}
