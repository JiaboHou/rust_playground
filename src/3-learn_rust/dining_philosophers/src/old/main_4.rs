use std::thread;

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    // 4th Iteration: Concurrency! All philosophers eat at the same time,
    // and finish at the same time.
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    // Create new threads, and provide us with handles with which to
    // control the operations of those theads.
    // Vec<_>: We use _ as a type placeholder.
    //         We can let Rust figure out the type.
    // philosophers.into_iter().map(): This creates an iterator that takes
    //         ownership of each philosopher. Required to pass them to
    //         the treads. Then, we call map on the iterator.
    //
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // This is where the concurrency happens.
        // thread::spawn takes a closure as an arg, and executes
        //         it in a new thread. This closure needs a 'move'
        //         annotation to indicate that closure is going to
        //         take ownership of the values it's capturing.
        //         (i.e. the p variable)`
        thread::spawn(move || {
            p.eat();
        }) // No semicolon to indicate that this ia an expression.
    }).collect(); // Turn the results of the map operation into a collection.
    // This is why we need to annotate the type in Vec<_>

    for h in handles {
        // Block execution until the thread has completed execution.
        // Thread will complete their work before program exits.
        h.join().unwrap();
    }
}
