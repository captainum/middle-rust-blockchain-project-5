use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    let bigi64: Vec<i64> = (0..50_000).collect();
    let bigu8: Vec<u8> = (0..u8::MAX).collect();
    let big_str: String = "  Hello World  ".repeat(600);

    // Несколько итераций, чтобы получить более подробные данные на flamegraph.
    for _ in 0..10000 {
        sum_even(&bigi64);

        leak_buffer(&bigu8);

        normalize(&big_str);

        algo::slow_fib(32);

        algo::slow_dedup(&dedup_data);
    }
}
