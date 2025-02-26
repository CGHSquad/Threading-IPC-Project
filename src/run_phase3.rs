use libc::{pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock};
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_phase3() {
    struct BankAccount {
        balance: i32,
        lock: pthread_mutex_t,
    }

    // Initialize two bank accounts
    let account1 = Arc::new(BankAccount {
        balance: 100,
        lock: unsafe {
            let mut mutex = MaybeUninit::<pthread_mutex_t>::uninit();
            pthread_mutex_init(mutex.as_mut_ptr(), std::ptr::null());
            mutex.assume_init()
        },
    });

    let account2 = Arc::new(BankAccount {
        balance: 200,
        lock: unsafe {
            let mut mutex = MaybeUninit::<pthread_mutex_t>::uninit();
            pthread_mutex_init(mutex.as_mut_ptr(), std::ptr::null());
            mutex.assume_init()
        },
    });

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    // First thread locks account1 -> tries to lock account2
    let thread1 = thread::spawn(move || {
        unsafe { pthread_mutex_lock(&acc1_clone.lock as *const _ as *mut _) };
        println!("Thread 1 locked Account 1");
        thread::sleep(Duration::from_secs(2));
        unsafe { pthread_mutex_lock(&acc2_clone.lock as *const _ as *mut _) };
        println!("Thread 1 locked Account 2");

        unsafe { pthread_mutex_unlock(&acc2_clone.lock as *const _ as *mut _) };
        unsafe { pthread_mutex_unlock(&acc1_clone.lock as *const _ as *mut _) };
    });

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    // Second thread locks account2 -> tries to lock account1
    let thread2 = thread::spawn(move || {
        unsafe { pthread_mutex_lock(&acc2_clone.lock as *const _ as *mut _) };
        println!("Thread 2 locked Account 2");
        thread::sleep(Duration::from_secs(2));
        unsafe { pthread_mutex_lock(&acc1_clone.lock as *const _ as *mut _) };
        println!("Thread 2 locked Account 1");

        unsafe { pthread_mutex_unlock(&acc1_clone.lock as *const _ as *mut _) };
        unsafe { pthread_mutex_unlock(&acc2_clone.lock as *const _ as *mut _) };
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    // Destroy the mutexes
    unsafe {
        pthread_mutex_destroy(&account1.lock as *const _ as *mut _);
        pthread_mutex_destroy(&account2.lock as *const _ as *mut _);
    }
}
pub(crate) fn run() {
    println!("Starting Phase 3...");
    run_phase3(); // Call the actual logic
    println!("Phase 3 Completed.");
}