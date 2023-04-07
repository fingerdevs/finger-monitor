// On Windows platform, don't show a console when opening the app.

pub mod chart;
pub mod models;
pub mod ui;

use std::time::Duration;

use sysinfo::{CpuExt, System, SystemExt};
use ui::run_app;

pub fn main() {
    run_app();

    // let mut sys = System::new_all();
    // sys.refresh_all();

    // loop {
    //     // RAM and swap information:
    //     println!("total memory: {} bytes", sys.total_memory());
    //     println!("used memory : {} bytes", sys.used_memory());
    //     println!("total swap  : {} bytes", sys.total_swap());
    //     println!("used swap   : {} bytes", sys.used_swap());

    //     // CPU iformation:
    //     let global_cpu = sys.global_cpu_info();
    //     println!("{} -> {}", global_cpu.name(), global_cpu.cpu_usage());
    //     for cpu in sys.cpus() {
    //         println!("{} -> {}", cpu.name(), cpu.cpu_usage())
    //     }

    //     std::thread::sleep(Duration::from_secs(1));
    //     sys.refresh_all();
    // }
}
