fn main() {
    let v = vec![5, 2, 9, 1, 3];
    println!("{:?}", merge_sort(&v));
}

fn merge<T: PartialOrd + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    match (left, right) {
        ([], r) => r.to_vec(),
        (l, []) => l.to_vec(),
        ([lh, lt @ ..], [rh, rt @ ..]) => {
            if lh <= rh {
                std::iter::once(lh.clone())
                    .chain(merge(lt, right))
                    .collect()
            } else {
                std::iter::once(rh.clone()).chain(merge(left, rt)).collect()
            }
        }
    }
}

fn merge_sort<T: PartialOrd + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] | [_] => list.to_vec(),
        _ => {
            let (left, right) = list.split_at(list.len() / 2);
            merge(&merge_sort(left), &merge_sort(right))
        }
    }
}
