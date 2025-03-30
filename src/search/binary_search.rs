#![allow(dead_code)]

use std::cmp::Ordering;

pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let (mut l, mut r) = (0, arr.len());
    while l < r {
        let m = l + (r - l) / 2;
        match arr[m].cmp(target) {
            Ordering::Less => l = m + 1,
            Ordering::Equal => return Some(m),
            Ordering::Greater => r = m,
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;

    #[test]
    fn test_binary_search() {
        {
            let arr: [i64; 10000] = std::array::from_fn(|i| i as i64 + 1);
            assert!(
                (1..=arr.len() as i64)
                    .into_par_iter()
                    .all(|target| binary_search(&arr, &target) == Some(target as usize - 1))
            );
            assert_eq!(binary_search(&arr, &0), None);
        }

        {
            let arr: Vec<i64> = vec![];
            assert_eq!(binary_search(&arr, &0), None);
        }

        {
            let arr = vec![1];
            assert_eq!(binary_search(&arr, &0), None);
            assert_eq!(binary_search(&arr, &1), Some(0));
        }
    }
}
