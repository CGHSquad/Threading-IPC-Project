use libc::{pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_phase2() {
    struct BankAccount {
        balance: i32,
    }

    // Shared bank account with a Mutex for safe mutability
    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let account_clone = Arc::clone(&account); // Clone Arc handle for each thread
        handles.push(thread::spawn(move || {
            // Lock the mutex before modifying the balance
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 10;
            println!("Updated Balance: {}", acc.balance);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Access the final balance after all threads are completed
    println!("Final Balance: {}", account.lock().unwrap().balance);
}
pub(crate) fn run() {
    println!("Starting Phase 2...");
    run_phase2(); // Call the actual logic
    println!("Phase 2 Completed.");
}