use std::collections::BTreeSet;
/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut tree = values.iter().map(|item| *item).collect::<BTreeSet<u64>>();
    let mut res = Vec::new();
    while let Some(val) = tree.pop_first() {
        res.push(val);
    }
    res
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
pub fn slow_fib(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let (mut a, mut b) = (0u64, 1u64);

    for _ in 2..(n + 1) {
        (a, b) = (b, a + b);
    }
    b
}
