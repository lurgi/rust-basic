mod async_basics;
mod async_collection;
mod future_composition;
mod timeout_select;

pub fn run_day19_exercises() {
    println!("\n========== Day 19: 비동기 프로그래밍 기초 ==========\n");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        async_basics::run().await;
        println!();
        future_composition::run().await;
        println!();
        timeout_select::run().await;
        println!();
        async_collection::run().await;
    });
}
