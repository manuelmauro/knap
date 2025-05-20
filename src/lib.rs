//! `knap` is a Rust crate for solving the knapsack problem using various algorithms.
//! It provides both an optimal solver (using dynamic programming) and a greedy approximation.
//! The crate is designed to be flexible, allowing users to define their own item types
//! by implementing the `Weight` and `Value` traits.
//!
//! # Features
//!
//! - Optimal knapsack solver (`KnapsackIterator`).
//! - Greedy knapsack solver (`GreedyKnapsackIterator`).
//! - Traits `Weight` and `Value` for custom item types.
//! - Extension traits `ToKnapsackIterator` and `ToGreedyKnapsackIterator` for easy solver creation
//!   from iterators.
//!
//! # Usage
//!
//! First, add `knap` to your `Cargo.toml` dependencies:
//!
//! ```toml
//! [dependencies]
//! knap = "0.1.0" # Replace with the latest version
//! ```
//!
//! Then, you can use the solvers in your code. Here's a simple example:
//!
//! ```
//! use knap::{GreedyKnapsackIterator, KnapsackIterator, Value, Weight};
//! use knap::traits::{ToKnapsackIterator, ToGreedyKnapsackIterator};
//!
//! #[derive(Clone, Debug, PartialEq)]
//! struct MyItem {
//!     id: String,
//!     weight: usize,
//!     value: usize,
//! }
//!
//! impl Weight for MyItem {
//!     fn weight(&self) -> usize {
//!         self.weight
//!     }
//! }
//!
//! impl Value for MyItem {
//!     fn value(&self) -> usize {
//!         self.value
//!     }
//! }
//!
//! fn main() {
//!     let items = vec![
//!         MyItem { id: "A".to_string(), weight: 10, value: 60 },
//!         MyItem { id: "B".to_string(), weight: 20, value: 100 },
//!         MyItem { id: "C".to_string(), weight: 30, value: 120 },
//!     ];
//!     let capacity = 50;
//!
//!     // Using the optimal solver
//!     println!("Optimal solution:");
//!     let optimal_iter = KnapsackIterator::new(items.clone(), capacity);
//!     for item in optimal_iter {
//!         println!("- {:?} (Weight: {}, Value: {})", item.id, item.weight(), item.value());
//!     }
//!
//!     // Using the greedy solver with the extension trait
//!     println!("\nGreedy solution:");
//!     let greedy_iter = items.to_greedy_knapsack_iter(capacity);
//!     for item in greedy_iter {
//!         println!("- {:?} (Weight: {}, Value: {})", item.id, item.weight(), item.value());
//!     }
//! }
//! ```
//!
//! ## Modules
//!
//! - `greedy`: Contains the `GreedyKnapsackIterator` for an approximate solution.
//! - `optimal`: Contains the `KnapsackIterator` for the optimal dynamic programming solution.
//! - `traits`: Contains the `Weight`, `Value`, `ToKnapsackIterator`, and `ToGreedyKnapsackIterator` traits.

pub mod greedy;
pub mod optimal;
pub mod traits;

pub use greedy::GreedyKnapsackIterator;
pub use optimal::KnapsackIterator;
pub use traits::{ToGreedyKnapsackIterator, ToKnapsackIterator, Value, Weight};
