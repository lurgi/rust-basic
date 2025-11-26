mod tokio_spawn_parallel;
mod async_error_handling;
mod http_client_simulator;
mod retry_logic;

pub fn run_day20_exercises() {
    println!("\n========== Day 20: 고급 비동기 프로그래밍 ==========\n");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio_spawn_parallel::run().await;
        println!();
        async_error_handling::run().await;
        println!();
        http_client_simulator::run().await;
        println!();
        retry_logic::run().await;
    });
}

