/*
 * The Dining Philosophers:
 * The classic concurrency problem.
 */

use std::thread;
use std::sync::{Mutex, Arc};

struct Philosopher {
    name: String,
    left: usize, // Vector index of left fork.
    right: usize, // Vector index of right fork.
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // Get access to the Mutex of the forks to the left and right
        // of the philosopher, if they are available. By calling lock,
        // we prevent any one else from accessing that Mutex.
        // If someone else is using the Mutex, a thread panic will
        // occur. We don't want this to happen, so we call unwrap().
        // We prepend underscore to bindings that we know we won't use.
        // That way Rust won't warn us about an unused binding.
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

struct Table {
    // A vector of Mutex's. Mutex's are used to control concurrency.
    // Only one thread can access the contents at once, which is the
    // exact requirement with our forks. () indicates an empty tuple,
    // since we don't need the value, we just need to hold onto it.
    forks: Vec<Mutex<()>>,
}

fn main() {
    // 5th Iteration: Final version!

    // Table is an "atomic reference count". We need to share Table
    // across multiple threads. As we share it, the reference count
    // increase, and when each thread ends, it will decrease.
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4), // We do 0, 4 to prevent a deadlock,
        // which is one way to solve the problem.
    ];

    // Create handles to threads.
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone(); // increase reference count.
        // reference count will automatically decrease when it
        // falls out of scope. That way, we can keep track of
        // the nuber of references to table exist across our
        // threads. Otherwise, we wouldn't know how to
        // deallocate it.
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        // Block execution until the thread has completed execution.
        // Thread will complete their work before program exits.
        h.join().unwrap();
    }
}
