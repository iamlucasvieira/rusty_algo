#[cfg(test)]
mod tests {
    use rusty_algo::sorting::SortAlgorithm::InsertionSort;
    use rusty_algo::sorting::Sortable;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![3, 2, 1];
        arr.insertion_sort();
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_sort() {
        let mut arr = vec![3, 2, 1];
        arr.sort(&InsertionSort);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}

