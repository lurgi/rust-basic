pub mod comparable_product;
pub mod conditional_methods;
pub mod logging_system;
pub mod shape_trait;

pub fn run_day13_exercises() {
    println!("\n========== Day 13: 트레잇과 제네릭 심화 ==========\n");

    shape_trait::run();
    println!();

    comparable_product::run();
    println!();

    logging_system::run();
    println!();

    conditional_methods::run();
}
