use std::sync::atomic::AtomicU64;
use std::thread;

static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Небезопасный инкремент через несколько потоков.
/// Использует global static mut без синхронизации — data race.
///
/// При добавлении тестов в [`tests/integration.rs`] запуск miri показал
/// следующие ошибки:
/// test multi_thread_increment_loses_updates ... error: Undefined Behavior: Data race detected between (1) non-atomic write on thread `unnamed-6` and (2) non-atomic read on thread `unnamed-7` at alloc69137
//   --> src/concurrency.rs:20:21
//    |
// 20 |                     COUNTER += 1;
//    |                     ^^^^^^^^^^^^ (2) just happened here
//    |
// help: and (1) occurred earlier here
//   --> src/concurrency.rs:20:21
//    |
// 20 |                     COUNTER += 1;
//    |                     ^^^^^^^^^^^^
//    = help: this indicates a bug in the program: it performed an invalid operation, and caused Undefined Behavior
//    = help: see https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html for further information
//    = note: this is on thread `unnamed-7`
// note: the current function got called indirectly due to this code
//   --> src/concurrency.rs:17:22
//    |
// 17 |           handles.push(thread::spawn(move || {
//    |  ______________________^
// 18 | |             for _ in 0..iterations {
// 19 | |                 unsafe {
// 20 | |                     COUNTER += 1;
// ...  |
// 23 | |         }));
//    | |__________^
//
// note: some details are omitted, run with `MIRIFLAGS=-Zmiri-backtrace=full` for a verbose backtrace
///
/// Что ожидаемо, так как наблюдается data race с переменной COUNTER при многопоточном взаимодействии.
/// Сделал её atomic. Добавлен [`thread::scope`], чтобы уйти от необходимости ручного вызова
/// join для созданных потоков.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    reset_counter();

    thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| {
                for _ in 0..iterations {
                    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            });
        }
    });

    read_after_sleep()
}

/// Плохая «синхронизация» — просто sleep, возвращает потенциально устаревшее значение.
///
/// Исправлен на безопасную реализацию.
pub fn read_after_sleep() -> u64 {
    COUNTER.load(std::sync::atomic::Ordering::SeqCst)
}

/// Сброс счётчика (также небезопасен, без синхронизации).
///
/// Исправлен на безопасную реализацию.
pub fn reset_counter() {
    COUNTER.store(0, std::sync::atomic::Ordering::SeqCst);
}
