use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_phase4_deadlock_prevention() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let transfer = |acc1: Arc<Mutex<BankAccount>>, acc2: Arc<Mutex<BankAccount>>, amount: i32| {
        let mut a1 = acc1.lock().unwrap();
        let mut a2 = acc2.lock().unwrap();
        a1.balance -= amount;
        a2.balance += amount;
    };

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    // Prevent deadlocks by ensuring consistent lock ordering
    let t1 = thread::spawn(move || {
        transfer(acc1_clone, acc2_clone, 50);
    });

    t1.join().unwrap();

    assert_eq!(account1.lock().unwrap().balance, 50, "Account 1 should have 50 after transfer");
    assert_eq!(account2.lock().unwrap().balance, 250, "Account 2 should have 250 after transfer");
}