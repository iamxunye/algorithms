#![allow(dead_code)]

use rand::Rng;
use std::cmp::Ordering;

pub fn top_k_min<T: Ord + Clone>(arr: &mut [T], k: usize) -> Option<T> {
    if k >= arr.len() {
        return None;
    }
    let pivot_val = arr[rand::rng().random_range(0..arr.len())].clone();
    let (lt, gt) = partition(arr, &pivot_val);
    if k < lt {
        top_k_min(&mut arr[..lt], k)
    } else if k >= gt {
        top_k_min(&mut arr[gt..], k - gt)
    } else {
        Some(arr[k].clone())
    }
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
    use rand::seq::SliceRandom;

    #[test]
    fn test_top_k_min() {
        {
            let mut random_arr: Vec<i64> = (0..1000).collect();
            random_arr.shuffle(&mut rand::rng());
            for i in 0..random_arr.len() {
                assert_eq!(top_k_min(&mut random_arr, i), Some(i as i64));
            }
        }

        {
            let mut arr: Vec<i64> = vec![];
            assert_eq!(top_k_min(&mut arr, 0), None);
            assert_eq!(top_k_min(&mut arr, 1), None);
        }

        {
            let mut arr = vec![0];
            assert_eq!(top_k_min(&mut arr, 0), Some(0));
            assert_eq!(top_k_min(&mut arr, 1), None);
        }
    }
}
