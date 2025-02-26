use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_phase2_protect_shared_resource() {
    struct BankAccount {
        balance: i32,
    }

    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let mut handles = vec![];

    // Simulate multiple threads
    for _ in 0..10 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 10;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_balance = account.lock().unwrap().balance;
    assert_eq!(final_balance, 200, "Final balance should be 200");
}