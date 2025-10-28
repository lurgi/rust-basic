// Day 9: 컬렉션 활용 과제
mod phone_book;
mod student_grades;
mod word_frequency;

pub fn run_day9_exercises() {
    println!("\n========================================");
    println!("        Day 9: 컬렉션 활용 과제");
    println!("========================================\n");

    student_grades::run();
    println!();
    word_frequency::run();
    println!();
    phone_book::run();

    println!("\n========================================");
    println!("        Day 9 완료!");
    println!("========================================\n");
}
