use hetero_rust::{init_log_info, sim::main_test, sim::mecch::MecchBuildContextFn};
use tracing::info;

fn main() {
    init_log_info();
    info!("mecch.rs");

    main_test::<MecchBuildContextFn>();
}
