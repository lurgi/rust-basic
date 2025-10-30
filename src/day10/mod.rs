// Day 10: Result와 Option 활용 과제
mod file_simulator;
mod safe_division;
mod student_registry;

pub fn run_day10_exercises() {
    println!("\n========================================");
    println!("        Day 10: Result와 Option 활용 과제");
    println!("========================================\n");

    safe_division::run();
    println!();
    student_registry::run();
    println!();
    file_simulator::run();

    println!("\n========================================");
    println!("        Day 10 완료!");
    println!("========================================\n");
}
