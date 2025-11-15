pub mod associated_types_container;
pub mod default_type_params;
pub mod trait_objects_notification;
pub mod combined_processor;

pub fn run_day15_exercises() {
    println!("\n========== Day 15: 고급 트레잇 (연관 타입, 기본 타입 매개변수, 트레잇 객체) ==========\n");

    associated_types_container::run();
    println!();

    default_type_params::run();
    println!();

    trait_objects_notification::run();
    println!();

    combined_processor::run();
}

