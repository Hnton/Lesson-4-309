extern crate rand;
use rand::Rng;


fn main() {
    let mut random_number = rand::thread_rng();
    let mut numbers = Vec::<i64>::with_capacity(100);
    
    for _ in 0..100 {
        numbers.push(random_number.gen_range(0,20));
    }
    println!("Random Number: {:#?}", numbers);
    println!("Size: {}", numbers.len());

}
