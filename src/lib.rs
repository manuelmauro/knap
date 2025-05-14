pub mod traits;
pub mod item;
pub mod knapsack;

pub use traits::{Value, Weight};
pub use item::Item;
pub use knapsack::KnapsackIterator;

// The main function is removed as this is a library now.
// The test module from the old main.rs will be placed here.

#[cfg(test)]
mod tests {
    // Adjusted to use crate level imports for clarity, 
    // or directly use re-exported types.
    use crate::item::Item; 
    use crate::knapsack::KnapsackIterator;
    use crate::traits::{Weight, Value};
    // Weight and Value traits are in scope via `crate::traits` or re-exports if needed by test logic directly.
    // Item struct implements them, so direct use in tests for Item is fine.

    #[test]
    fn test_case_1_basic() {
        let items1 = vec![
            Item { id: "item1".to_string(), weight: 2, value: 3 },
            Item { id: "item2".to_string(), weight: 3, value: 4 },
            Item { id: "item3".to_string(), weight: 4, value: 5 },
            Item { id: "item4".to_string(), weight: 5, value: 6 },
        ];
        let capacity1 = 7;

        // println! are fine for tests, they show up with `cargo test -- --nocapture`
        // println!("Items available (Test Case 1):");
        // for item in &items1 {
        //     println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
        // }
        // println!("Knapsack capacity: {}", capacity1);

        let knapsack_iter1 = KnapsackIterator::new(items1.clone(), capacity1);
        let mut total_weight1 = 0;
        let mut total_value1 = 0;
        let mut selected_ids1 = Vec::new();
        for item in knapsack_iter1 {
            // println!("  id: {}, weight: {}, value: {}", item.id, item.weight(), item.value());
            total_weight1 += item.weight();
            total_value1 += item.value();
            selected_ids1.push(item.id);
        }
        println!("Total weight of selected items: {}", total_weight1);
        // println!("Total value of selected items: {}", total_value1);
        assert_eq!(total_weight1, 7);
        assert_eq!(total_value1, 9);
        assert!(selected_ids1.contains(&"item2".to_string()));
        assert!(selected_ids1.contains(&"item3".to_string()));
        assert_eq!(selected_ids1.len(), 2);
    }

    #[test]
    fn test_case_2_empty_items() {
        let empty_items: Vec<Item> = Vec::new();
        let knapsack_iter_empty = KnapsackIterator::new(empty_items, 10);
        assert_eq!(knapsack_iter_empty.count(), 0, "Empty items should yield 0 selected items.");
    }

    #[test]
    fn test_case_3_zero_capacity() {
        let items_for_zero_cap = vec![Item { id: "itemA".to_string(), weight: 1, value: 10 }];
        let knapsack_iter_zero_cap = KnapsackIterator::new(items_for_zero_cap, 0);
        assert_eq!(knapsack_iter_zero_cap.count(), 0, "Zero capacity should yield 0 selected items.");
    }

    #[test]
    fn test_case_4_items_too_heavy() {
        let items_too_heavy = vec![
            Item { id: "heavy1".to_string(), weight: 10, value: 100 },
            Item { id: "heavy2".to_string(), weight: 12, value: 120 },
        ];
        let knapsack_iter_heavy = KnapsackIterator::new(items_too_heavy.clone(), 5);
        assert_eq!(knapsack_iter_heavy.count(), 0, "All items too heavy should yield 0 selected items.");
    }

    #[test]
    fn test_case_5_complex() {
        let items2 = vec![
            Item { id: "A".to_string(), weight: 10, value: 60 },
            Item { id: "B".to_string(), weight: 20, value: 100 },
            Item { id: "C".to_string(), weight: 30, value: 120 },
        ];
        let capacity2 = 50;
        let knapsack_iter2 = KnapsackIterator::new(items2.clone(), capacity2);
        let mut total_weight2 = 0;
        let mut total_value2 = 0;
        let mut selected_ids2 = Vec::new();
        for item in knapsack_iter2 {
            total_weight2 += item.weight();
            total_value2 += item.value();
            selected_ids2.push(item.id.clone());
        }
        assert_eq!(total_value2, 220, "Value for complex case is incorrect.");
        assert_eq!(total_weight2, 50, "Weight for complex case is incorrect.");
        assert!(selected_ids2.contains(&"B".to_string()), "Item B should be selected.");
        assert!(selected_ids2.contains(&"C".to_string()), "Item C should be selected.");
        assert_eq!(selected_ids2.len(), 2, "Incorrect number of items selected for complex case.");
    }

    #[test]
    fn test_case_6_zero_value_item() {
        let items3 = vec![
            Item { id: "valuable".to_string(), weight: 5, value: 10 },
            Item { id: "zero_val".to_string(), weight: 2, value: 0 },
        ];
        let capacity3 = 7;
        let knapsack_iter3 = KnapsackIterator::new(items3.clone(), capacity3);
        let mut total_weight3 = 0;
        let mut total_value3 = 0;
        let mut selected_ids3 = Vec::new();
        for item in knapsack_iter3 {
            total_weight3 += item.weight();
            total_value3 += item.value();
            selected_ids3.push(item.id.clone());
        }
        assert_eq!(total_value3, 10);
        assert_eq!(total_weight3, 5);
        assert!(selected_ids3.contains(&"valuable".to_string()));
        assert_eq!(selected_ids3.len(), 1);
    }
} 