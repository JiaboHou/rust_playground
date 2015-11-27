use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

fn main() {
    example_simple_concurrency();
    // Won't compile.
    // example_data_race_arc();
    example_data_race_mutex();
    example_data_race_channel();
    example_data_race_channel_send();
}

fn example_simple_concurrency() {
    let handle = thread::spawn(|| {
        "Hello from the other thread!"
    });

    println!("{}", handle.join().unwrap());
}

// Many other programming languages, when it comes to concurrency are concerned about data races.
// Rust on the other hand, due to its ownership system, avoids this.
// Note this will produce an error in compilation.
fn example_data_race() {
    let mut data = vec![1, 2, 3];

    for i in 0..3 {
        thread::spawn(move || {
            // This is unsafe. We would end up with a reference to data
            // in each thread and attempt to obtain ownership in each.
            data[i] += 1;
        });
    }

    thread::sleep_ms(50);
}

// We will need a type that lets us have more than one reference to a value
// that can be shared between threads.
// enter Sync
// We will use Arc<T>, which keeps track of the number of references to a value.
// Because this reference counter is atomic, we can access it safely from multiple threads.
fn example_data_race_arc() {
    let mut data = Arc::new(vec![1, 2, 3]);

    for i in 0..3 {
        // Increase internal reference count.
        let data = data.clone();
        thread::spawn(move || {
            // This still won't compile. Arc<T> assumes its contents are Sync.
            // This would be true if our value is immutable, but we are trying
            // to mutate it.
            data[i] += 1;
        });
    }

    thread::sleep_ms(50);
}

// We need a type that also allows for mutation of a shared value.
// So we need to ensure only one thread at a time is able to mutate the value.
// enter Mutex<T>
fn example_data_race_mutex() {
    // Wrap Mutex<T> within Arc<T>
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    for i in 0..3 {
        let data = data.clone();
        // Note that if we didn't wrap the Mutex in an Arc, we would
        // encounter another compiler error since we'd have lost Sync.
        thread::spawn(move || {
            // lock() acquires the mutex's lock, by returning a Result<T, E>
            // unwrap() gets the reference to the data.
            // (error handling for this is needed, but omitted for the purpose of
            // this example.)
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }

    thread::sleep_ms(50);
}


// What if we wanted to use a better mechanism than a timer to sync the threads?
// We'd either be waiting too long or not long enough for the most part.
// enter "channels"
fn example_data_race_channel() {
    let data = Arc::new(Mutex::new(0));

    // Construct a new channel.
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            // Sending a generic signal. We can send any data that is Send.
            tx.send(());
        });
    }

    for _ in 0..10 {
        rx.recv();
    }
}

// Send a Send-able type.
fn example_data_race_channel_send() {
    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let tx = tx.clone();

        thread::spawn(move || {
            // u32 is Send because we can create a copy.
            let answer = 42;

            tx.send((answer));
        });
    }

    rx.recv().ok().expect("Could not receive answer");
}

// Note: panic!'s will crash the currently executing thread. From this we
// can create a simple isolation mechanism.
let handle = thread::spawn(move || {
    panic!("oops!");
});

// Since this returns a Result, we can check if the thread panicked or not.
let result = handle.join();

assert!(result.is_err());
