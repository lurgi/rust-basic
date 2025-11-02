// Day 11: 에러 처리 심화
mod calculator_chaining;
mod data_pipeline;
mod user_registration;

pub fn run_day11_exercises() {
    println!("\n========================================");
    println!("        Day 11: 에러 처리 심화");
    println!("========================================\n");

    calculator_chaining::run();
    println!();
    user_registration::run();
    println!();
    data_pipeline::run();

    println!("\n========================================");
    println!("        Day 11 완료!");
    println!("========================================\n");
}
