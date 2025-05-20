use crate::traits::{Value, Weight};

#[derive(Debug)]
pub struct KnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    items: Vec<T>,
    capacity: usize,
    optimal_solution_items: Vec<T>,
    current_index: usize,
    // Ensures DP is run only once.
    computed: bool,
}

impl<T> KnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    pub fn new(input_items: impl IntoIterator<Item = T>, capacity: usize) -> Self {
        let items: Vec<T> = input_items.into_iter().collect();
        KnapsackIterator {
            items,
            capacity,
            optimal_solution_items: Vec::new(),
            current_index: 0,
            computed: false,
        }
    }

    // Computes the optimal solution using dynamic programming.
    fn compute_solution(&mut self) {
        let n = self.items.len();
        if n == 0 || self.capacity == 0 {
            self.computed = true;
            return;
        }

        let mut dp = vec![vec![0; self.capacity + 1]; n + 1];

        for i in 1..=n {
            let item_idx = i - 1;
            let item_weight = self.items[item_idx].weight();
            let item_value = self.items[item_idx].value();

            for w in 0..=self.capacity {
                let value_without_item = dp[i - 1][w];
                if item_weight <= w {
                    let value_with_item = dp[i - 1][w - item_weight] + item_value;
                    dp[i][w] = value_without_item.max(value_with_item);
                } else {
                    dp[i][w] = value_without_item;
                }
            }
        }

        let mut current_w = self.capacity;
        let mut solution_items_temp = Vec::new();

        for i in (1..=n).rev() {
            let item_idx = i - 1;
            let item_weight = self.items[item_idx].weight();

            if current_w >= item_weight && dp[i][current_w] != dp[i - 1][current_w] {
                solution_items_temp.push(self.items[item_idx].clone());
                current_w -= item_weight;
            }
        }

        solution_items_temp.reverse();
        self.optimal_solution_items = solution_items_temp;
    }
}

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

        let mut items_with_meta: Vec<(usize, f64)> = items_list
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
                        0.0
                    }
                };
                (idx, ratio)
            })
            .collect();

        items_with_meta.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut result_items = Vec::new();
        let mut current_capacity = capacity_val;

        for (original_idx, _ratio) in items_with_meta {
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
    fn to_greedy_knapsack_iter(self, capacity: usize) -> GreedyKnapsackIterator<T> {
        GreedyKnapsackIterator::new(self, capacity)
    }
}

impl<I, T> ToGreedyKnapsackIterator<T> for I
where
    I: IntoIterator<Item = T> + Sized,
    T: Weight + Value + Clone,
{
    // Use the default implementation from the trait.
}

impl<T> Iterator for KnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.computed {
            self.compute_solution();
            self.computed = true;
        }

        if self.current_index < self.optimal_solution_items.len() {
            let item = self.optimal_solution_items[self.current_index].clone();
            self.current_index += 1;
            Some(item)
        } else {
            None
        }
    }
}
