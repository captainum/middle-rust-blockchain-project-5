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
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
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
