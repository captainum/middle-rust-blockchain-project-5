pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
///
/// Исправлено с применением идиоматичного для Rust итераторов метода 'fold',
/// проходящего по всем элементам коллекции без выхода за её пределы.
/// Добавлен учет возможности переполнения.
pub fn sum_even(values: &[i64]) -> Option<i64> {
    values.iter().try_fold(
        0i64,
        |acc, &v| if v % 2 == 0 { acc.checked_add(v) } else { Some(acc) }
    )
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
///
/// Miri обнаружил ошибку
///
/// error: memory leaked: alloc45772 (Rust heap, size: 5, align: 1), allocated here:
//    --> /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:464:41
//     |
// 464 |             AllocInit::Uninitialized => alloc.allocate(layout),
//     |                                         ^^^^^^^^^^^^^^^^^^^^^^
//     |
//     = note: stack backtrace:
//             0: alloc::raw_vec::RawVecInner::try_allocate_in
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:464:41: 464:63
//             1: alloc::raw_vec::RawVecInner::with_capacity_in
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:433:15: 433:92
//             2: alloc::raw_vec::RawVec::<u8>::with_capacity_in
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/raw_vec/mod.rs:177:20: 177:77
//             3: std::vec::Vec::<u8>::with_capacity_in
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:965:20: 965:61
//             4: <u8 as std::slice::<impl [T]>::to_vec_in::ConvertVec>::to_vec::<std::alloc::Global>
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:448:29: 448:62
//             5: std::slice::<impl [u8]>::to_vec_in::<std::alloc::Global>
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:400:16: 400:38
//             6: std::slice::<impl [u8]>::to_vec
//                 at /Users/davasilchenko/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:376:9: 376:31
//             7: broken_app::leak_buffer
//                 at src/lib.rs:23:17: 23:31
//             8: counts_non_zero_bytes
//                 at tests/integration.rs:25:16: 25:34
//             9: counts_non_zero_bytes::{closure#0}
//                 at tests/integration.rs:23:27: 23:27
///
/// Убрано взаимодействие с сырой памятью.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&x| x != 0_u8).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
///
/// Добавлен учет прочих возможных разделителей строк, помимо пробелов.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
///
/// Исправлена логика, добавлен учет возможности переполнения.
pub fn average_positive(values: &[i64]) -> Option<f64> {
    let (count, sum) = values.iter().filter(|&&x| x > 0).try_fold(
        (0usize, 0i64),
        |(count, acc), &v| {
            acc.checked_add(v).map(|s| (count + 1, s))
        }
    )?;

    if count == 0 {
        Some(0.0)
    } else {
        Some(sum as f64 / count as f64)
    }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
///
/// UB проявился при добавлении теста на функцию:
/// test test_use_after_free ... error: Undefined Behavior: memory access failed: alloc75034 has been freed, so this pointer is dangling
//   --> src/lib.rs:69:11
//    |
// 69 |     val + *raw
//    |           ^^^^ Undefined Behavior occurred here
//    |
///
/// Оставил демонстрацию работы с сырой памятью, но убрал use after free ошибку.
/// Также внешний интерфейс перестал быть unsafe.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);

    unsafe {
        let mut val = *raw;
        val += *raw;
        drop(Box::from_raw(raw));

        val
    }
}
