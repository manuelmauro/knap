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
        if self.computed {
            return;
        }

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
            
            if current_w >= item_weight && dp[i][current_w] != dp[i-1][current_w] {
                solution_items_temp.push(self.items[item_idx].clone());
                current_w -= item_weight;
            }
        }
        
        solution_items_temp.reverse();
        self.optimal_solution_items = solution_items_temp;
        self.computed = true;
    }
}

impl<T> Iterator for KnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.computed {
            self.compute_solution();
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