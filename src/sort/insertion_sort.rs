#![allow(dead_code)]

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_insertion_sort() {
        {
            let mut random_arr1: [i64; 1000] =
                std::array::from_fn(|_| rand::rng().random_range(0..100));
            let mut random_arr2 = random_arr1;
            insertion_sort(&mut random_arr1);
            random_arr2.sort();
            assert_eq!(random_arr1, random_arr2);
        }

        {
            let mut arr: [i64; 0] = [];
            insertion_sort(&mut arr);
            assert_eq!(arr, []);
        }

        {
            let mut arr: [i64; 1] = [1];
            insertion_sort(&mut arr);
            assert_eq!(arr, [1]);
        }
    }
}
