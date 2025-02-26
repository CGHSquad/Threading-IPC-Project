use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_phase1() {
    struct BankAccount {
        balance: i32,
    }

    // Shared account
    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));

    let mut handles = vec![];

    // Simulate deposit
    for _ in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 50;
            println!("Deposited 50, Balance: {}", acc.balance);
        }));
    }

    // Simulate withdrawal
    for _ in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance -= 20;
            println!("Withdrew 20, Balance: {}", acc.balance);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Balance: {}", account.lock().unwrap().balance);
}