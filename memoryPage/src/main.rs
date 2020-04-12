extern crate rand;
use rand::Rng;
mod fifo();

#[derive(Debug)]
pub struct Page {
    number: i64,
    pub reference: bool
}

impl Page {
    pub fn new(ref_number: i64) -> Page {
        Page {
            number: ref_number,
            reference: false
        }
    }
    fn update_ref(&mut self){
        self.reference = true;
    }
}

pub struct Report {
    pub hits: i64,
    pub removes: i64
    pub faults: i64
}

impl Report{
    pub fn new() -> Report
    Report{
        hits: 0,
        removes: 0,
        faults: 0
    }
    fn update_hits(&mut self, hit: &i64 ){
        self.hits = hit; 
    }
    fn update_removes(&mut self, remove: &i64 ){
        self.removes = remove; 
    }
    fn update_faults(&mut self, fault: &i64 ){
        self.faults = fault; 
    }
}


fn main() {

    // Buffer size (Manually change for now to get 3, 5, 10)
    let buffer_size = 3;

    // Random number generator
    let mut random_number = rand::thread_rng();

    // Creating Vec of 100 numbers
    let mut ref_string = Vec::<i64>::with_capacity(100);
    
    // Pushing 100 random numbers from 0 - 19 into Vec
    for _ in 0..100 {
        ref_string.push(random_number.gen_range(0,20));
    }

    // Cloning numbers
    let mut numbers_clone = ref_string.clone();

    // Creating buffer
    let mut buffer = Vec::<i64>::with_capacity(buffer_size);

    // Prints original 100 numbers
    println!("Original: {:#?}", ref_string);

    // Pushes buffer_size from 100 numbers into buffer
    for i in 0..buffer_size {
        buffer.push(numbers_clone[i]);
    }

    // Creates buffer clone
    let mut buffer_clone = buffer.clone();

    // Removes buffer_size from 100 numbers
    for n in 0..buffer_size {
        numbers_clone.remove(0);
    }

    // Prints buffer at start
    println!("buffer {:#?}", buffer_clone);

    // REPLACE IN BUFFER

    // Removes index 0 from buffer 
    buffer_clone.remove(0);
    // Inserts index 0 from 100 numbers into index 0 from buffer
    buffer_clone.insert(0, numbers_clone[0]);
    // Removes index 0 from 100 numbers
    numbers_clone.remove(0);

    println!("Changed buffer: {:#?}", buffer_clone);

    // let mut p = Page::new(numbers_clone[0]);
    // print!("{:#?}", p);
    // p.update_ref();
    // print!("{:#?}", p);


}

pub fn fifo(num: i64)
{
    let buffer_size = 3;
    let mut frame_buffer = Vec::<i64>::with_capacity(buffer_size);


}

pub fn nru()
{

}

pub fn lru()
{

}

pub fn second_chance()
{

}

pub fn clock()
{

}