pub mod basic_threads;
pub mod channel_messaging;
pub mod mutex_counter;
pub mod producer_consumer;

pub fn run_day18_exercises() {
    println!("\n========== Day 18: 동시성 (Concurrency) ==========\n");

    basic_threads::run();
    println!();

    channel_messaging::run();
    println!();

    mutex_counter::run();
    println!();

    producer_consumer::run();
}
