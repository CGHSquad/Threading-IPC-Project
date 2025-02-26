use std::sync::{Arc, Mutex};
use std::thread;
use libc;
use std::ffi::CString;

// BankAccount structure with balance
struct BankAccount {
    balance: i32,
}

/// Sets a thread name using `prctl` for better identification in Linux tools.
unsafe fn set_thread_name(name: &str) {
    let cname = CString::new(name).expect("CString::new failed");
    libc::prctl(libc::PR_SET_NAME, cname.as_ptr() as u64, 0, 0, 0);
}

/// Simulate deposits and withdrawals using threads
pub fn run_phase1() {
    // Shared account with initial balance
    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let mut handles = vec![];

    // Simulate deposits using threads
    for i in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            unsafe {
                set_thread_name(&format!("deposit-{}", i + 1)); // Set thread name
            }
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 50; // Deposit operation
            println!("Deposited 50, Balance: {}", acc.balance);
        }));
    }

    // Simulate withdrawals using threads
    for i in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            unsafe {
                set_thread_name(&format!("withdraw-{}", i + 1)); // Set thread name
            }
            let mut acc = account_clone.lock().unwrap();
            acc.balance -= 20; // Withdrawal operation
            println!("Withdrew 20, Balance: {}", acc.balance);
        }));
    }

    // Join all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final balance after all threads are done
    println!("Final Balance: {}", account.lock().unwrap().balance);
}

/// Startup function
pub(crate) fn run() {
    println!("Starting Phase 1...");
    run_phase1(); // Call the main logic
    println!("Phase 1 Completed.");
}