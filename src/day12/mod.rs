// Day 12: 제네릭 프로그래밍
mod generic_storage;
mod generic_utilities;
mod wrapper_struct;

pub fn run_day12_exercises() {
    println!("\n========================================");
    println!("        Day 12: 제네릭 프로그래밍");
    println!("========================================\n");

    generic_utilities::run();
    println!();
    wrapper_struct::run();
    println!();
    generic_storage::run();

    println!("\n========================================");
    println!("        Day 12 완료!");
    println!("========================================\n");
}
