// Day 8: 모듈 시스템 과제
mod calculator_module;
mod config_management;
mod library_system;

pub fn run_day8_exercises() {
    println!("\n========================================");
    println!("        Day 8: 모듈 시스템 과제");
    println!("========================================\n");

    library_system::run();
    println!();
    calculator_module::run();
    println!();
    config_management::run();

    println!("\n========================================");
    println!("        Day 8 완료!");
    println!("========================================\n");
}
