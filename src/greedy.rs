use crate::traits::{Value, Weight};

/// An iterator that yields items based on a greedy approximation
/// for the knapsack problem.
///
/// This iterator computes the greedy solution upon creation and then
/// yields the selected items one by one.
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
                } else {
                    if value > 0 {
                        std::f64::MAX
                    } else {
                        -1.0
                    }
                };
                (idx, ratio, value)
            })
            .collect();

        items_with_meta.sort_by(|a, b| {
            let ratio_cmp = b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal);
            if ratio_cmp == std::cmp::Ordering::Equal {
                if a.1 == std::f64::MAX && b.1 == std::f64::MAX {
                    b.2.cmp(&a.2)
                } else {
                    a.0.cmp(&b.0)
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
    /// * `input_items`: An iterator over items that implement `Weight`, `Value`, and `Clone`.
    /// * `capacity`: The maximum capacity of the knapsack.
    pub fn new(input_items: impl IntoIterator<Item = T>, capacity: usize) -> Self {
        let items_vec: Vec<T> = input_items.into_iter().collect();
        // Use the new internal static method for greedy calculation
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
