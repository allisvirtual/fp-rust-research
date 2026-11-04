//! Functions as first-class citizens: closures, iterators and higher-order functions.

fn analyse_transactions<F>(
    transactions: &[i32],
    predicate: F,
    multiplier: i32
) -> Vec<i32>
where
    F: Fn(&i32) -> bool,
{
    transactions
        .iter()
        .filter(|n| predicate(n))
        .map(|n| n * multiplier)
        .collect()
}

pub(crate) fn demo() {
    let transactions = [45, -34, 163];

    let result = analyse_transactions(&transactions,
                                      |n| *n >= 0,
                                      2
    );

    println!("{:?}", result);
}