use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Note that if you are doing simple numerical operations, there are types
    // simpler than Mutex<T> types provided by the std::sync::atomic module of 
    // the standard library. These types provide safe, concurrent, atomic access 
    // to primitive types. We chose to use Mutex<T> with a primitive type for this 
    // example so we could concentrate on how Mutex<T> works.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
