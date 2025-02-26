use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_phase2() {
    struct BankAccount {
        balance: i32,
    }

    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));

    let mut handles = vec![];

    // Protect access to shared balance using Mutex
    for _ in 0..10 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 10;
            println!("Updated Balance: {}", acc.balance);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Balance: {}", account.lock().unwrap().balance);
}