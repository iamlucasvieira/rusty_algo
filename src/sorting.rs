pub trait Sortable {
    fn sort(&mut self, algorithm: &SortAlgorithm);
    fn insertion_sort(&mut self);
}

impl<T: Ord> Sortable for Vec<T> {
    fn sort(&mut self, algorithm: &SortAlgorithm) {
        algorithm.sort(self);
    }

    fn insertion_sort(&mut self) {
        self.sort(&SortAlgorithm::InsertionSort);
    }
}
pub enum SortAlgorithm {
    InsertionSort,
}

impl SortAlgorithm {
    fn sort<T: Ord>(&self, arr: &mut [T]) {
        match &self {
            SortAlgorithm::InsertionSort => self.insertion_sort(arr),
        }
    }

    fn insertion_sort<T: Ord>(&self, arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
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
}
