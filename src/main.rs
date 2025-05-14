// Trait for items that have a weight
trait Weight {
    fn weight(&self) -> usize;
}

// Trait for items that have a value
trait Value {
    fn value(&self) -> usize;
}

// Example item struct
#[derive(Debug, Clone)]
struct Item {
    id: String,
    weight: usize,
    value: usize,
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

struct KnapsackIterator<T>
where
    T: Weight + Value + Clone,
{
    items: Vec<T>,
    capacity: usize,
    optimal_solution_items: Vec<T>,
    current_index: usize,
    computed: bool, // To ensure DP is run only once
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

    // Helper method to compute the optimal solution using dynamic programming
    fn compute_solution(&mut self) {
        if self.computed {
            return;
        }

        let n = self.items.len();
        if n == 0 || self.capacity == 0 {
            self.computed = true;
            return;
        }

        // dp[i][w] stores the maximum value using the first i items with capacity w.
        // Dimensions: (n+1) x (capacity+1)
        let mut dp = vec![vec![0; self.capacity + 1]; n + 1];

        for i in 1..=n { // Corresponds to item self.items[i-1]
            let item_idx = i - 1;
            let item_weight = self.items[item_idx].weight();
            let item_value = self.items[item_idx].value();

            for w in 0..=self.capacity {
                // Option 1: Don't include item i (self.items[item_idx])
                let value_without_item = dp[i - 1][w];

                // Option 2: Include item i (self.items[item_idx]), if it fits
                if item_weight <= w {
                    let value_with_item = dp[i - 1][w - item_weight] + item_value;
                    dp[i][w] = value_without_item.max(value_with_item);
                } else {
                    // Item doesn't fit, so we can only take the value without it
                    dp[i][w] = value_without_item;
                }
            }
        }

        // Reconstruct the solution items
        // Start from the last cell dp[n][capacity]
        let mut current_w = self.capacity;
        let mut solution_items_temp = Vec::new();

        for i in (1..=n).rev() { // Iterate from n down to 1 (for dp table rows)
            let item_idx = i - 1; // Corresponding 0-indexed item in self.items
            let item_weight = self.items[item_idx].weight();
            // let item_value = self.items[item_idx].value(); // Not directly needed for reconstruction with this logic

            // If dp[i][current_w] is different from dp[i-1][current_w],
            // it means item i (self.items[item_idx]) was included in the solution for this state.
            if current_w >= item_weight && dp[i][current_w] != dp[i-1][current_w] {
                 // This condition implies that taking the item was better:
                 // dp[i][current_w] == dp[i-1][current_w - item_weight] + item_value
                 // AND dp[i-1][current_w - item_weight] + item_value > dp[i-1][current_w]
                solution_items_temp.push(self.items[item_idx].clone());
                current_w -= item_weight;
            }
            // If dp[i][current_w] == dp[i-1][current_w], item i was not taken.
            // current_w remains the same for iteration i-1.
        }
        
        solution_items_temp.reverse(); // Items were added in order from n down to 1. Reverse to get a more natural order.
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

fn main() {
    let items1 = vec![
        Item { id: "item1".to_string(), weight: 2, value: 3 },
        Item { id: "item2".to_string(), weight: 3, value: 4 },
        Item { id: "item3".to_string(), weight: 4, value: 5 },
        Item { id: "item4".to_string(), weight: 5, value: 6 },
    ];
    let capacity1 = 7;

    println!("Items available (Test Case 1):");
    for item in &items1 {
        println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
    }
    println!("Knapsack capacity: {}", capacity1);

    let knapsack_iter1 = KnapsackIterator::new(items1.clone(), capacity1);
    println!("
Optimal items in knapsack (Test Case 1):");
    let mut total_weight1 = 0;
    let mut total_value1 = 0;
    let mut selected_ids1 = Vec::new();
    for item in knapsack_iter1 {
        println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
        total_weight1 += item.weight();
        total_value1 += item.value();
        selected_ids1.push(item.id);
    }
    println!("
Total weight of selected items: {}", total_weight1);
    println!("Total value of selected items: {}", total_value1);
    // Expected for (items1, capacity1=7): Item2 (3,4) + Item3 (4,5) -> Weight 7, Value 9
    assert_eq!(total_weight1, 7);
    assert_eq!(total_value1, 9);
    assert!(selected_ids1.contains(&"item2".to_string()));
    assert!(selected_ids1.contains(&"item3".to_string()));

    // Test with empty items
    let empty_items: Vec<Item> = Vec::new();
    let knapsack_iter_empty = KnapsackIterator::new(empty_items, 10);
    println!("
Testing with empty items (Test Case 2):");
    assert_eq!(knapsack_iter_empty.count(), 0, "Empty items should yield 0 selected items.");
    println!("Correctly yielded 0 items for empty input.");

    // Test with zero capacity
    let items_for_zero_cap = vec![Item { id: "itemA".to_string(), weight: 1, value: 10 }];
    let knapsack_iter_zero_cap = KnapsackIterator::new(items_for_zero_cap, 0);
    println!("
Testing with zero capacity (Test Case 3):");
    assert_eq!(knapsack_iter_zero_cap.count(), 0, "Zero capacity should yield 0 selected items.");
    println!("Correctly yielded 0 items for zero capacity.");

    // Test with items that are all too heavy
    let items_too_heavy = vec![
        Item { id: "heavy1".to_string(), weight: 10, value: 100 },
        Item { id: "heavy2".to_string(), weight: 12, value: 120 },
    ];
    let knapsack_iter_heavy = KnapsackIterator::new(items_too_heavy.clone(), 5);
    println!("
Testing with items too heavy for capacity 5 (Test Case 4):");
    let mut count_heavy = 0;
    for item in knapsack_iter_heavy {
        println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value()); // Should not print
        count_heavy += 1;
    }
    assert_eq!(count_heavy, 0, "All items too heavy should yield 0 selected items.");
    println!("Correctly yielded 0 items when all items are too heavy.");

    // More complex case
    let items2 = vec![
        Item { id: "A".to_string(), weight: 10, value: 60 },
        Item { id: "B".to_string(), weight: 20, value: 100 },
        Item { id: "C".to_string(), weight: 30, value: 120 },
    ];
    let capacity2 = 50;
    // Expected: B (w:20, v:100) + C (w:30, v:120) => Total W:50, Total V:220
    println!("
Complex test case (Items: A(10,60), B(20,100), C(30,120), Capacity: 50) (Test Case 5):");
    let knapsack_iter2 = KnapsackIterator::new(items2.clone(), capacity2);
    let mut total_weight2 = 0;
    let mut total_value2 = 0;
    let mut selected_ids2 = Vec::new();
    for item in knapsack_iter2 {
        println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
        total_weight2 += item.weight();
        total_value2 += item.value();
        selected_ids2.push(item.id.clone());
    }
    println!("  Total weight: {}, Total value: {}", total_weight2, total_value2);
    assert_eq!(total_value2, 220, "Value for complex case is incorrect.");
    assert_eq!(total_weight2, 50, "Weight for complex case is incorrect.");
    assert!(selected_ids2.contains(&"B".to_string()), "Item B should be selected.");
    assert!(selected_ids2.contains(&"C".to_string()), "Item C should be selected.");
    assert_eq!(selected_ids2.len(), 2, "Incorrect number of items selected for complex case.");
    println!("Complex test case passed.");

    // Test case with an item of value 0 that could fit
    let items3 = vec![
        Item { id: "valuable".to_string(), weight: 5, value: 10 },
        Item { id: "zero_val".to_string(), weight: 2, value: 0 },
    ];
    let capacity3 = 7;
    // Expected: "valuable" (5,10). "zero_val" might be included if it doesn't prevent "valuable".
    // If "valuable" is chosen (w:5, v:10), remaining capacity is 2. "zero_val" (w:2, v:0) fits.
    // DP[valuable][w=5] = 10. DP[valuable][w=7] = 10.
    // Item "zero_val":
    //   DP[zero_val_then_valuable][w=7] = max(DP[valuable][7], DP[valuable][7-2] + 0)
    //                                   = max(10, DP[valuable][5] + 0)
    //                                   = max(10, 10 + 0) = 10.
    // Reconstruction:
    // i="zero_val": current_w=7. dp[zero_val_...][7](10) == dp[valuable][7](10). Not taken. (Correct by rule)
    // i="valuable": current_w=7. dp[valuable][7](10) != dp[empty][7](0). Taken. selected=["valuable"], current_w=2.
    // Result should be just ["valuable"].
    println!("
Test case with zero-value item (Test Case 6):");
    let knapsack_iter3 = KnapsackIterator::new(items3.clone(), capacity3);
    let mut total_weight3 = 0;
    let mut total_value3 = 0;
    let mut selected_ids3 = Vec::new();
    for item in knapsack_iter3 {
        println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
        total_weight3 += item.weight();
        total_value3 += item.value();
        selected_ids3.push(item.id.clone());
    }
    println!("  Total weight: {}, Total value: {}", total_weight3, total_value3);
    assert_eq!(total_value3, 10);
    assert_eq!(total_weight3, 5);
    assert!(selected_ids3.contains(&"valuable".to_string()));
    assert_eq!(selected_ids3.len(), 1);
    println!("Zero-value item test case passed.");
}

#[cfg(test)]
mod tests {
    use super::*; // Make items from parent module visible

    #[test]
    fn test_case_1_basic() {
        let items1 = vec![
            Item { id: "item1".to_string(), weight: 2, value: 3 },
            Item { id: "item2".to_string(), weight: 3, value: 4 },
            Item { id: "item3".to_string(), weight: 4, value: 5 },
            Item { id: "item4".to_string(), weight: 5, value: 6 },
        ];
        let capacity1 = 7;

        println!("Items available (Test Case 1):");
        for item in &items1 {
            println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
        }
        println!("Knapsack capacity: {}", capacity1);

        let knapsack_iter1 = KnapsackIterator::new(items1.clone(), capacity1);
        println!("
Optimal items in knapsack (Test Case 1):");
        let mut total_weight1 = 0;
        let mut total_value1 = 0;
        let mut selected_ids1 = Vec::new();
        for item in knapsack_iter1 {
            println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
            total_weight1 += item.weight();
            total_value1 += item.value();
            selected_ids1.push(item.id);
        }
        println!("
Total weight of selected items: {}", total_weight1);
        println!("Total value of selected items: {}", total_value1);
        // Expected for (items1, capacity1=7): Item2 (3,4) + Item3 (4,5) -> Weight 7, Value 9
        assert_eq!(total_weight1, 7);
        assert_eq!(total_value1, 9);
        assert!(selected_ids1.contains(&"item2".to_string()));
        assert!(selected_ids1.contains(&"item3".to_string()));
    }

    #[test]
    fn test_case_2_empty_items() {
        let empty_items: Vec<Item> = Vec::new();
        let knapsack_iter_empty = KnapsackIterator::new(empty_items, 10);
        println!("
Testing with empty items (Test Case 2):");
        assert_eq!(knapsack_iter_empty.count(), 0, "Empty items should yield 0 selected items.");
        println!("Correctly yielded 0 items for empty input.");
    }

    #[test]
    fn test_case_3_zero_capacity() {
        let items_for_zero_cap = vec![Item { id: "itemA".to_string(), weight: 1, value: 10 }];
        let knapsack_iter_zero_cap = KnapsackIterator::new(items_for_zero_cap, 0);
        println!("
Testing with zero capacity (Test Case 3):");
        assert_eq!(knapsack_iter_zero_cap.count(), 0, "Zero capacity should yield 0 selected items.");
        println!("Correctly yielded 0 items for zero capacity.");
    }

    #[test]
    fn test_case_4_items_too_heavy() {
        let items_too_heavy = vec![
            Item { id: "heavy1".to_string(), weight: 10, value: 100 },
            Item { id: "heavy2".to_string(), weight: 12, value: 120 },
        ];
        let knapsack_iter_heavy = KnapsackIterator::new(items_too_heavy.clone(), 5);
        println!("
Testing with items too heavy for capacity 5 (Test Case 4):");
        let mut count_heavy = 0;
        for item in knapsack_iter_heavy {
            println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value()); // Should not print
            count_heavy += 1;
        }
        assert_eq!(count_heavy, 0, "All items too heavy should yield 0 selected items.");
        println!("Correctly yielded 0 items when all items are too heavy.");
    }

    #[test]
    fn test_case_5_complex() {
        let items2 = vec![
            Item { id: "A".to_string(), weight: 10, value: 60 },
            Item { id: "B".to_string(), weight: 20, value: 100 },
            Item { id: "C".to_string(), weight: 30, value: 120 },
        ];
        let capacity2 = 50;
        // Expected: B (w:20, v:100) + C (w:30, v:120) => Total W:50, Total V:220
        println!("
Complex test case (Items: A(10,60), B(20,100), C(30,120), Capacity: 50) (Test Case 5):");
        let knapsack_iter2 = KnapsackIterator::new(items2.clone(), capacity2);
        let mut total_weight2 = 0;
        let mut total_value2 = 0;
        let mut selected_ids2 = Vec::new();
        for item in knapsack_iter2 {
            println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
            total_weight2 += item.weight();
            total_value2 += item.value();
            selected_ids2.push(item.id.clone());
        }
        println!("  Total weight: {}, Total value: {}", total_weight2, total_value2);
        assert_eq!(total_value2, 220, "Value for complex case is incorrect.");
        assert_eq!(total_weight2, 50, "Weight for complex case is incorrect.");
        assert!(selected_ids2.contains(&"B".to_string()), "Item B should be selected.");
        assert!(selected_ids2.contains(&"C".to_string()), "Item C should be selected.");
        assert_eq!(selected_ids2.len(), 2, "Incorrect number of items selected for complex case.");
        println!("Complex test case passed.");
    }

    #[test]
    fn test_case_6_zero_value_item() {
        let items3 = vec![
            Item { id: "valuable".to_string(), weight: 5, value: 10 },
            Item { id: "zero_val".to_string(), weight: 2, value: 0 },
        ];
        let capacity3 = 7;
        println!("
Test case with zero-value item (Test Case 6):");
        let knapsack_iter3 = KnapsackIterator::new(items3.clone(), capacity3);
        let mut total_weight3 = 0;
        let mut total_value3 = 0;
        let mut selected_ids3 = Vec::new();
        for item in knapsack_iter3 {
            println!("  Selected: id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
            total_weight3 += item.weight();
            total_value3 += item.value();
            selected_ids3.push(item.id.clone());
        }
        println!("  Total weight: {}, Total value: {}", total_weight3, total_value3);
        assert_eq!(total_value3, 10);
        assert_eq!(total_weight3, 5);
        assert!(selected_ids3.contains(&"valuable".to_string()));
        assert_eq!(selected_ids3.len(), 1);
        println!("Zero-value item test case passed.");
    }
}
