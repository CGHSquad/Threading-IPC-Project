use std::sync::{Arc, Mutex};
use std::thread;

#[test]
#[should_panic] // Expected to fail due to deadlock
fn test_phase3_deadlock_scenario() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let t1 = thread::spawn(move || {
        let _lock1 = acc1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(2));
        let _lock2 = acc2_clone.lock().unwrap();
    });

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let t2 = thread::spawn(move || {
        let _lock2 = acc2_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(2));
        let _lock1 = acc1_clone.lock().unwrap();
    });

    let _ = t1.join();
    let _ = t2.join();
}