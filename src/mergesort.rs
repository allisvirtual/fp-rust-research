//! Mergesort in Rust, using list recursion and pattern matching.
//! Owner: Alea Munnik
use std::iter::once;

pub fn demo() {
    let v = vec![5, 2, 9, 1, 3];
    println!("merge_sort   {:?}", merge_sort(&v));
}

pub fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    match (left, right) {
        ([], _) => right.to_vec(),
        (_, []) => left.to_vec(),
        ([first_left, rest_left @ ..], [first_right, rest_right @ ..]) => {
            if first_left <= first_right {
                once(first_left.clone())
                    .chain(merge(rest_left, right))
                    .collect()
            } else {
                once(first_right.clone())
                    .chain(merge(left, rest_right))
                    .collect()
            }
        }
    }
}

pub fn merge_sort<T: PartialOrd + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] | [_] => list.to_vec(),
        _ => {
            let (left, right) = list.split_at(list.len() / 2);
            merge(&merge_sort(left), &merge_sort(right))
        }
    }
}
