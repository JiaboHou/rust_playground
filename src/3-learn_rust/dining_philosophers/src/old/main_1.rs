struct Philosopher {
    name: String,
}

impl Philosopher {
    // Constructor: Basically a static function.
    //  :param name: pointer to a string.
    //  :return: A Philosopher.
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

fn main() {
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Mara");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel Foucault");
}
