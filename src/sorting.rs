//! # Sorting
//! This module contains sorting algorithms.
//! ## Example
//! Using sorting algorithm method directly:
//! ```rust
//! use rusty_algo::sorting::Sortable;
//! let mut arr = vec![3, 2, 1];
//! arr.insertion_sort();
//! assert_eq!(arr, vec![1, 2, 3]);
//! ```
//!
//! Using sorting algorithm enum:
//! ```rust
//! use rusty_algo::sorting::SortAlgorithm::InsertionSort;
//! use rusty_algo::sorting::Sortable;
//! let mut arr = vec![3, 2, 1];
//! arr.sort_with(&InsertionSort);
//! assert_eq!(arr, vec![1, 2, 3]);
//! ```
//!
//! ## Sorting algorithms
//! - Insertion sort

/// Sortable trait for sorting algorithms
pub trait Sortable {
    /// Sorts the vector in place using the specified algorithm
    fn sort_with(&mut self, algorithm: &SortAlgorithm);

    /// Sorts the vector in place using the insertion sort algorithm
    fn insertion_sort(&mut self);
    fn merge_sort(&mut self);
}

/// Implement Sortable for Vec<T> where T is an Ord
impl<T: Ord + Clone> Sortable for [T] {
    fn sort_with(&mut self, algorithm: &SortAlgorithm) {
        algorithm.sort(self);
    }

    fn insertion_sort(&mut self) {
        self.sort_with(&SortAlgorithm::InsertionSort);
    }

    fn merge_sort(&mut self) {
        self.sort_with(&SortAlgorithm::MergeSort);
    }
}

/// SortAlgorithm enum
pub enum SortAlgorithm {
    InsertionSort,
    MergeSort,
}

/// Implement SortAlgorithm
impl SortAlgorithm {
    /// Sorts the array in place using the specified algorithm
    fn sort<T: Ord + Clone>(&self, arr: &mut [T]) {
        match &self {
            SortAlgorithm::InsertionSort => SortAlgorithm::insertion_sort(arr),
            SortAlgorithm::MergeSort => SortAlgorithm::merge_sort(arr),
        }
    }

    /// Sorts the array in place using the insertion sort algorithm
    fn insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }

    /// Sorts the array in place using the merge sort algorithm
    fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }

        let mid = len / 2;
        let (left, right) = arr.split_at_mut(mid);

        SortAlgorithm::merge_sort(left);
        SortAlgorithm::merge_sort(right);
        let mut temp = Vec::with_capacity(len);

        let len_left = left.len();
        let len_right = right.len();

        let mut idx_left = 0;
        let mut idx_right = 0;

        while idx_left < len_left && idx_right < len_right {
            let left_val = &left[idx_left];
            let right_val = &right[idx_right];

            if left_val < right_val {
                temp.push(left_val.clone());
                idx_left += 1;
            } else {
                temp.push(right_val.clone());
                idx_right += 1;
            }
        }

        temp.extend_from_slice(&left[idx_left..]);
        temp.extend_from_slice(&right[idx_right..]);

        arr.clone_from_slice(&temp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase<T> {
        input: Vec<T>,
        expected: Vec<T>,
        description: &'static str,
    }

    fn test_cases_i32() -> Vec<TestCase<i32>> {
        vec![
            TestCase {
                input: vec![3, 2, 1],
                expected: vec![1, 2, 3],
                description: "Reverse sorted",
            },
            TestCase {
                input: vec![4, 5, 6],
                expected: vec![4, 5, 6],
                description: "Sorted",
            },
            TestCase {
                input: vec![7, 9, 8],
                expected: vec![7, 8, 9],
                description: "Unsorted",
            },
        ]
    }

    fn test_cases_string() -> Vec<TestCase<String>> {
        vec![
            TestCase {
                input: vec!["c".to_string(), "b".to_string(), "a".to_string()],
                expected: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                description: "Reverse sorted",
            },
            TestCase {
                input: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                expected: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                description: "Sorted",
            },
            TestCase {
                input: vec!["a".to_string(), "c".to_string(), "b".to_string()],
                expected: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                description: "Unsorted",
            },
        ]
    }

    fn test_algorithm<T, F>(test_cases: Vec<TestCase<T>>, sort_fn: F)
    where
        T: Ord + Clone + std::fmt::Debug,
        F: Fn(&mut Vec<T>),
    {
        for test_case in test_cases {
            let mut input = test_case.input.clone();
            sort_fn(&mut input);
            assert_eq!(input, test_case.expected, "{}", test_case.description);
        }
    }

    #[test]
    fn test_insertion_sort_i32() {
        test_algorithm(test_cases_i32(), |arr| arr.insertion_sort());
    }

    #[test]
    fn test_insertion_sort_string() {
        test_algorithm(test_cases_string(), |arr| arr.insertion_sort());
    }

    #[test]
    fn test_merge_sort_i32() {
        test_algorithm(test_cases_i32(), |arr| {
            arr.sort_with(&SortAlgorithm::MergeSort)
        });
    }

    #[test]
    fn test_merge_sort_string() {
        test_algorithm(test_cases_string(), |arr| {
            arr.sort_with(&SortAlgorithm::MergeSort)
        });
    }
}
