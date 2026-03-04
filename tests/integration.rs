use broken_app::{algo, leak_buffer, normalize, sum_even, use_after_free};

#[test]
fn sums_even_numbers() {
    let nums = [1, 2, 3, 4];
    // Ожидаем корректное суммирование: 2 + 4 = 6.
    assert_eq!(sum_even(&nums), Some(6));

    let nums = [];
    // Проверка передачи пустого массива.
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [1, 3, 5];
    // Проверка передачи массива без четных значений.
    assert_eq!(sum_even(&nums), Some(0));

    let nums = [i64::MAX - 1, i64::MAX - 1];
    // Проверка учета переполнения
    assert_eq!(sum_even(&nums), None);
}

#[test]
fn counts_non_zero_bytes() {
    let data = [0_u8, 1, 0, 2, 3];
    assert_eq!(leak_buffer(&data), 3);

    let data = [];
    assert_eq!(leak_buffer(&data), 0);

    let data = [0, 0, 0, 0];
    assert_eq!(leak_buffer(&data), 0);
}

#[test]
fn dedup_preserves_uniques() {
    // Пустой срез — пустой результат.
    assert_eq!(algo::slow_dedup(&[]), Vec::<u64>::new());

    // Один элемент — возвращается как есть.
    assert_eq!(algo::slow_dedup(&[42]), vec![42]);

    // Уже уникальные элементы — сортировка без потерь.
    assert_eq!(algo::slow_dedup(&[3, 1, 2]), vec![1, 2, 3]);

    // Все элементы одинаковые — остаётся один.
    assert_eq!(algo::slow_dedup(&[7, 7, 7]), vec![7]);

    // Дубликаты удалены, результат отсортирован.
    assert_eq!(algo::slow_dedup(&[5, 5, 1, 2, 2, 3]), vec![1, 2, 3, 5]);
}

#[test]
fn fib_small_numbers() {
    // Базовые случаи рекурсии.
    assert_eq!(algo::slow_fib(0), Some(0));
    assert_eq!(algo::slow_fib(1), Some(1));

    // Произвольное значение из середины.
    assert_eq!(algo::slow_fib(10), Some(55));

    // Последнее значение, умещающееся в u64.
    assert_eq!(algo::slow_fib(92), Some(7540113804746346429));

    // fib(93) переполняет u64 — ожидаем None.
    assert_eq!(algo::slow_fib(93), None);
}

#[test]
fn normalize_simple() {
    assert_eq!(normalize(" Hello World "), "helloworld");
    assert_eq!(
        normalize(" Mary   had\ta\u{2009}little  \n\t lamb"),
        "maryhadalittlelamb"
    );
}

#[test]
fn averages_only_positive() {
    let nums = [-5, 5, 15];
    assert!((broken_app::average_positive(&nums).unwrap() - 10.0).abs() < f64::EPSILON);

    let nums = [];
    assert_eq!(broken_app::average_positive(&nums), Some(0.0));

    let nums = [-5, -5, -15];
    assert_eq!(broken_app::average_positive(&nums), Some(0.0));

    let nums = [i64::MAX, i64::MAX];
    assert_eq!(broken_app::average_positive(&nums), None);
}

#[test]
fn test_use_after_free() {
    assert_eq!(use_after_free(), 84);
}

#[test]
fn multi_thread_race_increment() {
    let threads = 4;
    let iterations = 1000;
    let result = broken_app::concurrency::race_increment(iterations, threads);
    assert_eq!(result, 4000);
}
