use std::{thread, time::Duration};
fn track_changes() {
    let mut counter = 0;

    let mut update = || {
        counter += 1;
        println!("Counter: {}", counter);
    };

    update();
    update();
    update();
}
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // --- Version 1: using map ---
    vec.into_iter().map(f).collect()

    // --- Version 2: for loop (alternative) ---
    /*
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
    */
}
struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}

fn main() {
    // Task 1
    let operation = |a: i32, b: i32| a * b;
    println!("Result: {}", operation(10, 5));

    // Task 2
    track_changes();
    // Task 3
let numbers = vec![1, 2, 3];

let doubled = process_vector(numbers.clone(), |x| x * 2);

let replaced = process_vector(numbers, |x| {
    if x > 2 { 0 } else { x }
});

// Task 5
let mut cache = ComputeCache::new(|| {
    println!("Computing (this will take 2 seconds)...");
    thread::sleep(Duration::from_secs(2));
    "Hello, world!".to_string()
});
println!("Doubled: {:?}", doubled);
println!("Replaced: {:?}", replaced);

println!("First call:");
println!("Result: {}", cache.get_result());

println!("\nSecond call:");
println!("Result (cached): {}", cache.get_result());


}