pub mod complex_lifetimes;
pub mod lifetime_basics;
pub mod lifetime_struct;
pub mod parser_struct;

pub fn run_day16_exercises() {
    println!("\n========== Day 16: 생명주기 (Lifetimes) ==========\n");

    lifetime_basics::run();
    println!();

    lifetime_struct::run();
    println!();

    complex_lifetimes::run();
    println!();

    parser_struct::run();
}
