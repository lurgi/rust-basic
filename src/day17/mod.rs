pub mod box_linked_list;
pub mod rc_refcell_shared_mutable;
pub mod rc_shared_data;
pub mod refcell_interior_mutability;

pub fn run_day17_exercises() {
    println!("\n========== Day 17: 스마트 포인터 (Smart Pointers) ==========\n");

    box_linked_list::run();
    println!();

    rc_shared_data::run();
    println!();

    refcell_interior_mutability::run();
    println!();

    rc_refcell_shared_mutable::run();
}




