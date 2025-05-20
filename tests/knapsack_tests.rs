use knap::traits::{KnapsackIterableExt, Value, Weight};

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub weight: usize,
    pub value: usize,
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

#[test]
fn test_basic() {
    let items = vec![
        Item {
            id: "item1".to_string(),
            weight: 2,
            value: 3,
        },
        Item {
            id: "item2".to_string(),
            weight: 3,
            value: 4,
        },
        Item {
            id: "item3".to_string(),
            weight: 4,
            value: 5,
        },
        Item {
            id: "item4".to_string(),
            weight: 5,
            value: 6,
        },
    ];
    let capacity = 7;

    let knapsack_iter = items.clone().to_knapsack_iter(capacity);
    let mut total_weight = 0;
    let mut total_value = 0;
    let mut selected_ids = Vec::new();
    for item in knapsack_iter {
        total_weight += item.weight();
        total_value += item.value();
        selected_ids.push(item.id);
    }
    println!("Total weight of selected items: {}", total_weight);
    assert_eq!(total_weight, 7);
    assert_eq!(total_value, 9);
    assert!(selected_ids.contains(&"item2".to_string()));
    assert!(selected_ids.contains(&"item3".to_string()));
    assert_eq!(selected_ids.len(), 2);
}

#[test]
fn test_empty_items() {
    let empty_items: Vec<Item> = Vec::new();
    let knapsack_iter_empty = empty_items.to_knapsack_iter(10);
    assert_eq!(
        knapsack_iter_empty.count(),
        0,
        "Empty items should yield 0 selected items."
    );
}

#[test]
fn test_zero_capacity() {
    let items_for_zero_cap = vec![Item {
        id: "itemA".to_string(),
        weight: 1,
        value: 10,
    }];
    let knapsack_iter_zero_cap = items_for_zero_cap.to_knapsack_iter(0);
    assert_eq!(
        knapsack_iter_zero_cap.count(),
        0,
        "Zero capacity should yield 0 selected items."
    );
}

#[test]
fn test_items_too_heavy() {
    let items_too_heavy = vec![
        Item {
            id: "heavy1".to_string(),
            weight: 10,
            value: 100,
        },
        Item {
            id: "heavy2".to_string(),
            weight: 12,
            value: 120,
        },
    ];
    let knapsack_iter_heavy = items_too_heavy.clone().to_knapsack_iter(5);
    assert_eq!(
        knapsack_iter_heavy.count(),
        0,
        "All items too heavy should yield 0 selected items."
    );
}

#[test]
fn test_complex() {
    let items = vec![
        Item {
            id: "A".to_string(),
            weight: 10,
            value: 60,
        },
        Item {
            id: "B".to_string(),
            weight: 20,
            value: 100,
        },
        Item {
            id: "C".to_string(),
            weight: 30,
            value: 120,
        },
    ];
    let capacity = 50;
    let knapsack_iter = items.clone().to_knapsack_iter(capacity);
    let mut total_weight = 0;
    let mut total_value = 0;
    let mut selected_ids = Vec::new();
    for item in knapsack_iter {
        total_weight += item.weight();
        total_value += item.value();
        selected_ids.push(item.id.clone());
    }
    assert_eq!(total_value, 220, "Value for complex case is incorrect.");
    assert_eq!(total_weight, 50, "Weight for complex case is incorrect.");
    assert!(
        selected_ids.contains(&"B".to_string()),
        "Item B should be selected."
    );
    assert!(
        selected_ids.contains(&"C".to_string()),
        "Item C should be selected."
    );
    assert_eq!(
        selected_ids.len(),
        2,
        "Incorrect number of items selected for complex case."
    );
}

#[test]
fn test_zero_value_item() {
    let items = vec![
        Item {
            id: "valuable".to_string(),
            weight: 5,
            value: 10,
        },
        Item {
            id: "zero_val".to_string(),
            weight: 2,
            value: 0,
        },
    ];
    let capacity = 7;
    let knapsack_iter = items.clone().to_knapsack_iter(capacity);
    let mut total_weight = 0;
    let mut total_value = 0;
    let mut selected_ids = Vec::new();
    for item in knapsack_iter {
        total_weight += item.weight();
        total_value += item.value();
        selected_ids.push(item.id.clone());
    }
    assert_eq!(total_value, 10);
    assert_eq!(total_weight, 5);
    assert!(selected_ids.contains(&"valuable".to_string()));
    assert_eq!(selected_ids.len(), 1);
}
