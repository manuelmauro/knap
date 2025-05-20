# Knapsack Iterator Crate

This Rust crate provides an iterator-based solution to the 0/1 knapsack problem. It allows you to find the subset of items that maximizes total value while staying within a given weight capacity.

## Features

-   **Iterator-based Solution**: Lazily computes and yields the items in the optimal knapsack solution.
-   **Generic**: Works with any item type that implements the `Weight` and `Value` traits.
-   **Ergonomic API**: Includes an extension trait `KnapsackIterableExt` for easily creating a `KnapsackIterator` from any compatible iterable collection (e.g., `Vec<YourItemType>`).
-   **Dynamic Programming**: Uses a standard dynamic programming approach to solve the 0/1 knapsack problem.

## Installation

Add this crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
knap = { git = "https://github.com/manuelmauro/knap" }
```

Then, run `cargo build`.

## Usage

First, ensure your item type implements the `Weight` and `Value` traits from this crate.

```rust
// src/main.rs or your library/binary
use knap::{KnapsackIterator, ToKnapsackIterator, Weight, Value};

// Define your custom item or use the provided `knap::Item`
// For demonstration, we'll use the provided `Item` struct.
// If you have your own struct:
#[derive(Debug, Clone)]
struct MyCustomItem {
    name: String,
    cost: usize, // represents weight
    worth: usize, // represents value
}

impl Weight for MyCustomItem {
    fn weight(&self) -> usize {
        self.cost
    }
}

impl Value for MyCustomItem {
    fn value(&self) -> usize {
        self.worth
    }
}

fn main() {
    let items = vec![
        MyCustomItem { name: "item1".to_string(), cost: 10, worth: 60 },
        MyCustomItem { name: "item2".to_string(), cost: 20, worth: 100 },
        MyCustomItem { name: "item3".to_string(), cost: 30, worth: 120 },
    ];
    let capacity = 50;

    println!("Available items:");
    for item in &items {
        println!("  Name: {}, Cost: {}, Worth: {}", item.name, item.weight(), item.value());
    }
    println!("Knapsack Capacity: {}", capacity);

    // You can create the iterator using the constructor:
    // let knapsack_solution = KnapsackIterator::new(items.clone(), capacity);

    // Or, more ergonomically, using the extension trait:
    // (Ensure ToKnapsackIterator is in scope)
    let knapsack_solution = items.to_knapsack_iter(capacity);

    println!("Optimal items in the knapsack:");
    let mut total_weight = 0;
    let mut total_value = 0;
    for item in knapsack_solution {
        println!("  Selected: Name: {}, Cost: {}, Worth: {}", item.name, item.weight(), item.value());
        total_weight += item.weight();
        total_value += item.value();
    }

    println!("Total weight of selected items: {}", total_weight);
    println!("Total value of selected items: {}", total_value);

    // Expected output for the above example:
    // Name: item3, Cost: 30, Worth: 120
    // Name: item2, Cost: 20, Worth: 100
    // Total weight: 50
    // Total value: 220
}
```

## Running Tests

To run the tests included with the crate:

```bash
cargo test
```

This will execute all unit tests.

## License

This project is licensed under the MIT license OR Apache License, Version 2.0.
