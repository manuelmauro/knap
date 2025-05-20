#[cfg(test)]
mod actual_tests {
    // Assuming your crate is named "knap"
    // and your modules are publicly accessible as knap::optimal and knap::traits
    use knap::greedy::GreedyKnapsackIterator;
    use knap::traits::{ToGreedyKnapsackIterator, Value, Weight};

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct TestItem {
        id: String,
        weight: usize,
        value: usize,
    }

    impl TestItem {
        fn new(id: &str, weight: usize, value: usize) -> Self {
            TestItem {
                id: id.to_string(),
                weight,
                value,
            }
        }
    }

    impl Weight for TestItem {
        fn weight(&self) -> usize {
            self.weight
        }
    }

    impl Value for TestItem {
        fn value(&self) -> usize {
            self.value
        }
    }

    #[test]
    fn greedy_empty_items() {
        let items: Vec<TestItem> = Vec::new();
        let capacity = 10;
        let mut iter = GreedyKnapsackIterator::new(items.clone(), capacity);
        assert_eq!(iter.next(), None);

        // Test with extension trait
        let mut iter_ext = items.to_greedy_knapsack_iter(capacity);
        assert_eq!(iter_ext.next(), None);
    }

    #[test]
    fn greedy_zero_capacity() {
        let items = vec![TestItem::new("A", 10, 100)];
        let capacity = 0;
        let mut iter = GreedyKnapsackIterator::new(items.clone(), capacity);
        assert_eq!(iter.next(), None);

        let mut iter_ext = items.to_greedy_knapsack_iter(capacity);
        assert_eq!(iter_ext.next(), None);
    }

    #[test]
    fn greedy_basic_selection() {
        let items = vec![
            TestItem::new("A", 10, 60),  // Ratio 6
            TestItem::new("B", 20, 100), // Ratio 5
            TestItem::new("C", 30, 120), // Ratio 4
        ];
        let capacity = 50;
        let mut iter = GreedyKnapsackIterator::new(items.clone(), capacity);

        assert_eq!(iter.next(), Some(TestItem::new("A", 10, 60)));
        assert_eq!(iter.next(), Some(TestItem::new("B", 20, 100)));
        assert_eq!(iter.next(), None);

        let mut iter_ext = items.to_greedy_knapsack_iter(capacity);
        assert_eq!(iter_ext.next(), Some(TestItem::new("A", 10, 60)));
        assert_eq!(iter_ext.next(), Some(TestItem::new("B", 20, 100)));
        assert_eq!(iter_ext.next(), None);
    }

    #[test]
    fn greedy_all_items_fit() {
        let items = vec![TestItem::new("A", 10, 60), TestItem::new("B", 20, 100)];
        let capacity = 30;
        let mut iter = GreedyKnapsackIterator::new(items.clone(), capacity);
        assert_eq!(iter.next(), Some(TestItem::new("A", 10, 60)));
        assert_eq!(iter.next(), Some(TestItem::new("B", 20, 100)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn greedy_no_items_fit() {
        let items = vec![TestItem::new("A", 60, 100)];
        let capacity = 50;
        let mut iter = GreedyKnapsackIterator::new(items, capacity);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn greedy_items_with_same_ratio_order_by_input() {
        let items = vec![
            TestItem::new("A", 10, 100),
            TestItem::new("B", 10, 100),
            TestItem::new("C", 5, 40),
        ];
        let capacity = 15;
        let mut iter = GreedyKnapsackIterator::new(items, capacity);

        let mut results = vec![iter.next(), iter.next()];
        results.sort_by_key(|item| item.clone().map(|i| i.id));

        assert_eq!(results[0], Some(TestItem::new("A", 10, 100)));
        assert_eq!(results[1], Some(TestItem::new("C", 5, 40)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn greedy_item_with_zero_weight_positive_value() {
        let items = vec![
            TestItem::new("FreeGood", 0, 1000),
            TestItem::new("A", 10, 60),
        ];
        let capacity = 10;
        let mut iter = GreedyKnapsackIterator::new(items, capacity);
        assert_eq!(iter.next(), Some(TestItem::new("FreeGood", 0, 1000)));
        assert_eq!(iter.next(), Some(TestItem::new("A", 10, 60)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn greedy_item_with_zero_weight_zero_value() {
        let items = vec![
            TestItem::new("WorthlessFree", 0, 0),
            TestItem::new("A", 10, 60),
        ];

        let mut iter_cap10 = GreedyKnapsackIterator::new(items.clone(), 10);
        assert_eq!(iter_cap10.next(), Some(TestItem::new("A", 10, 60)));
        assert_eq!(
            iter_cap10.next(),
            Some(TestItem::new("WorthlessFree", 0, 0))
        );
        assert_eq!(iter_cap10.next(), None);

        let items_only_worthless = vec![
            TestItem::new("WorthlessFree", 0, 0),
            TestItem::new("Big", 100, 100),
        ];
        let mut iter_small_cap = GreedyKnapsackIterator::new(items_only_worthless, 5);
        assert_eq!(
            iter_small_cap.next(),
            Some(TestItem::new("WorthlessFree", 0, 0))
        );
        assert_eq!(iter_small_cap.next(), None);
    }

    #[test]
    fn greedy_multiple_zero_weight_items() {
        let items = vec![
            TestItem::new("A", 10, 10),
            TestItem::new("Free1", 0, 100),
            TestItem::new("Free2", 0, 200),
        ];
        let capacity = 10;
        let mut iter = GreedyKnapsackIterator::new(items, capacity);

        assert_eq!(iter.next(), Some(TestItem::new("Free2", 0, 200)));
        assert_eq!(iter.next(), Some(TestItem::new("Free1", 0, 100)));
        assert_eq!(iter.next(), Some(TestItem::new("A", 10, 10)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn greedy_complex_case_with_zero_weights() {
        let items = vec![
            TestItem::new("ItemA", 20, 100),
            TestItem::new("ItemB", 30, 120),
            TestItem::new("FreeValuable", 0, 50),
            TestItem::new("ItemC", 10, 65),
            TestItem::new("FreeWorthless", 0, 0),
        ];
        let capacity = 50;

        let mut iter = GreedyKnapsackIterator::new(items.clone(), capacity);

        assert_eq!(iter.next(), Some(TestItem::new("FreeValuable", 0, 50)));
        assert_eq!(iter.next(), Some(TestItem::new("ItemC", 10, 65)));
        assert_eq!(iter.next(), Some(TestItem::new("ItemA", 20, 100)));
        assert_eq!(iter.next(), Some(TestItem::new("FreeWorthless", 0, 0)));
        assert_eq!(iter.next(), None);

        let mut iter_ext = items.to_greedy_knapsack_iter(capacity);
        assert_eq!(iter_ext.next(), Some(TestItem::new("FreeValuable", 0, 50)));
        assert_eq!(iter_ext.next(), Some(TestItem::new("ItemC", 10, 65)));
        assert_eq!(iter_ext.next(), Some(TestItem::new("ItemA", 20, 100)));
        assert_eq!(iter_ext.next(), Some(TestItem::new("FreeWorthless", 0, 0)));
        assert_eq!(iter_ext.next(), None);
    }
}
