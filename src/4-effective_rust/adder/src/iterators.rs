fn main() {
    // Basic for loop.
    for x in 0..10 {
        println!("{}", x);
    }

    // Using .next()
    let mut range = 0..10;

    loop {
        match range.next() {
            Some(x) => {
                println!("{}", x);
            },
            None => {break}
        }
    }

    let nums = vec![1, 2, 3];
    // DO:
    for num in &nums {
        // Note: num here is actually of type &i32. println will dereference for us.
        println!("{}", num);
    }
    // DON'T:
    for i in 0..nums.len() {
        println!("{}", nums[i]);
    }

    // Consumers

    // collect()
    // Note: "::<>" syntax allows us to provide a type hint.
    // Here, we want a vector of integers.
    let one_to_one_hundred = (1..101).collect::<Vec<i32>>();
    // This also works:
    let one_to_one_hundred_one = (1..102).collect::<Vec<_>>();

    // find()
    // Returns an Option, in case we don't find a matching element.
    let greater_than_forty_two = (0..100).find(|x| * x > 42);

    match greater_than_forty_two {
        Some(_) => println!("We got some numbers!"),
        None => println!("No numbers found :("),
    }

    // fold()
    // Take the sum of the values in nums
    nums.fold(0, |sum, x| sum + x);

    // Iterators

    // Ranges
    // This doesn't actually generate the sequence, since we didn't do anything with it.
    let more_nums_iter = 1..100;
    // Now that we're collecting these values, we need to generate the sequence.
    let more_nums = (1..100).collect::<Vec<i32>>();

    // iter()
    // Turn a vector into a basic iterator.
    for num in nums.iter() {
        println("{}", num);
    }

    // Iterator Adapters

    // map()
    (1..100).map(|x| println!("{}", x + 1));

    // take()
    // Takes the next n elements of an iterator and makes an iterator from that.
    // Note the use an infinite iterator.
    // Output:
    // 1
    // 2
    // 3
    // 4
    // 5
    for i in (1..).take(5) {
        println!("{}", i);
    }

    // filter()
    // Note that filter does not mutate the elements, so we pass a reference in the closure.
    for i in (1..100).filter(|&x| x % 2 == 0) {
        println!("{}", i);
    }


    // Example
    // The first 5 numbers divisible by 2 and 3.
    let nums_multiple_2_3 = (1..)
        .filter(|&x| x % 2 == 0)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect::<Vec<i32>>();
}
