#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;

pub fn quick_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() < 2 {
        return;
    }
    let pivot_val = arr[rand::rng().random_range(0..arr.len())].clone();
    let (lt, gt) = partition(arr, &pivot_val);
    quick_sort(&mut arr[..lt]);
    quick_sort(&mut arr[gt..]);
}

fn partition<T: Ord + Clone>(arr: &mut [T], pivot_val: &T) -> (usize, usize) {
    let (mut lt, mut gt) = (0, arr.len());
    let mut i = 0;
    while i < gt {
        match arr[i].cmp(pivot_val) {
            Ordering::Less => {
                arr.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => i += 1,
            Ordering::Greater => {
                gt -= 1;
                arr.swap(i, gt);
            }
        }
    }
    (lt, gt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        {
            let mut random_arr1: Vec<i64> = (0..10000)
                .map(|_| rand::rng().random_range(0..1000))
                .collect();
            let mut random_arr2 = random_arr1.clone();
            quick_sort(&mut random_arr1);
            random_arr2.sort();
            assert_eq!(random_arr1, random_arr2);
        }

        {
            let mut arr: Vec<i64> = vec![];
            quick_sort(&mut arr);
            assert_eq!(arr, []);
        }

        {
            let mut arr = vec![1];
            quick_sort(&mut arr);
            assert_eq!(arr, [1]);
        }
    }
}
