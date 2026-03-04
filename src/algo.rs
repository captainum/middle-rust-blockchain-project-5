use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/// Намеренно низкопроизводительная реализация.
///
/// Добавлено кеширование уже обнаруженных элементов (HashMap для поиска за O(1)),
/// а также было сокращено количество сортировок до одной, перед возвратом результирующего значения.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for &v in values {
        if seen.insert(v) {
            out.push(v);
        }
    }
    out.sort_unstable();

    out
}

thread_local! {
    static FIB_CACHE: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::from([(0, 0), (1, 1)]));
}

/// Итеративная реализация с мемоизацией результатов между вызовами.
///
/// Повторные вызовы и вызовы с близкими значениями n используют кешированные
/// промежуточные результаты. Возвращает None при переполнении u64.
///
/// Добавлено кеширование для уже посчитанных значений, что в целом дает улучшение
/// производительности по мере наполнения кэша увеличивающимися со временем значениями, что со
/// временем приведет к возможности функции сразу возвращать уже закешированное значение без
/// необходимости проведения дополнительных вычислений.
///
/// Локальный для потока кэш при необходимости можно сделать и разделяемым между потоками
/// с использованием механизмов синхронизации (так как со временем операций чтения станет много
/// больше операций записи, лучше подойдет RwLock).
pub fn slow_fib(n: u64) -> Option<u64> {
    if let Some(val) = FIB_CACHE.with(|c| c.borrow().get(&n).copied()) {
        return Some(val);
    }

    let (mut a, mut b, best_m) = FIB_CACHE.with(|c| {
        let cache = c.borrow();
        let best_m = cache
            .keys()
            .filter(|&&k| k < n && cache.contains_key(&(k + 1)))
            .copied()
            .max()
            .unwrap_or(0);
        let a = *cache.get(&best_m).unwrap();
        let b = *cache.get(&(best_m + 1)).unwrap();
        (a, b, best_m)
    });

    for k in 1..=(n - best_m) {
        let next = a.checked_add(b)?;
        a = b;
        b = next;
        FIB_CACHE.with(|c| c.borrow_mut().insert(best_m + k, a));
    }

    Some(a)
}
