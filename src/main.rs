
mod run_phase2;
mod run_phase1;
mod run_phase3;
mod run_phase4;
// Import module for IPC phase

fn main() {
    // Simple menu to select which phase to execute
    println!("Select a phase to run:");
    println!("1: Phase 1 - Basic Threading");
    println!("2: Phase 2 - Threading with Mutex");
    println!("3: Phase 3 - Deadlock Simulation");
    println!("4: Phase 4 - IPC with Pipes");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => threading_phase_1::run(),
        "2" => threading_phase_2::run(),
        "3" => threading_phase_3::run(),
        "4" => threading_phase_4::run(),
        "5" => ipc_phase::run(),
        _ => println!("Invalid choice! Please choose 1, 2, 3, or 4."),
    }
}