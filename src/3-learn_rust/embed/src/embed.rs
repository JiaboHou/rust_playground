use std::thread;

// Allow process to be callable by C.
// pub => allow this function to be called outside this module
// extern => allow this function to be called from C.

// When you create a Rust library, it changes the name of the fn
// in the compiled output. We disable that with "#[no_mangle]"

#[no_mangle]
pub extern fn process() {
    let handles: Vec<_> = (0..10).map(|_| {
        thread::spawn(|| {
            let mut x = 0;
            for _ in (0..5_000_000) {
                x += 1
            }
            x
        })
    }).collect();

    for h in handles {
        println!("Thread finished with count={}",
            h.join().map_err(|_| "Could not join a thread!").unwrap());
    }
    println!("done!");
}
