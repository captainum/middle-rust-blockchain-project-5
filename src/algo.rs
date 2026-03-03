/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // бесполезная сортировка на каждой вставке
        }
    }
    out
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
///
/// Добавлена обработка случаев переполнения значений u64 (Option::None при больших значениях) и
/// переполнений стека вызовов (уход от рекурсии).
pub fn slow_fib(n: u64) -> Option<u64> {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let next = a.checked_add(b)?;
        a = b;
        b = next;
    }
    Some(a)
}
