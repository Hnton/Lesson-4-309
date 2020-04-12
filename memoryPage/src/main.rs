extern crate rand;
use rand::Rng;
use std::clone::Clone;
use std::collections::VecDeque;
const BUFFER_SIZE: usize = 3; 



#[derive(Copy, Clone, Debug)]
pub struct Page {
    pub number: i64,
    pub reference: bool
}

impl Page {
    pub fn new(ref_number: i64) -> Page {
        Page {
            number: ref_number,
            reference: false
        }
    }
    pub fn update_ref(&mut self){
        self.reference = true;
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

#[derive(Debug)]
pub struct Report {
    pub hits: i64,
    pub removes: i64,
    pub faults: i64
}

impl Report{
    pub fn new() -> Report {
        Report{
            hits: 0,
            removes: 0,
            faults: 0
        }
    }
    fn update_hits(&mut self){
        self.hits = self.hits + 1; 
    }
    fn update_removes(&mut self){
        self.removes = self.removes + 1; 
    }
    fn update_faults(&mut self){
        self.faults = self.faults + 1; 
    }
}


fn main() {

    // Buffer size (Manually change for now to get 3, 5, 10)
    let buffer_size = 3;

    // Random number generator
    let mut random_number = rand::thread_rng();

    // Creating Vec of 100 numbers
    let mut ref_string = Vec::<Page>::new();
    
    // Pushing 100 random numbers from 0 - 19 into Vec
    for _ in 0..100 {
        ref_string.push(Page::new(random_number.gen_range(0,20)));
    }

    // Cloning numbers
    let mut numbers_clone = ref_string.clone();

    // Creating buffer
    let mut buffer = Vec::<i64>::with_capacity(buffer_size);

    // Prints original 100 numbers
    println!("Original: {:#?}", ref_string);

    // // Pushes buffer_size from 100 numbers into buffer
    // for i in 0..buffer_size {
    //     buffer.push(numbers_clone[i]);
    // }

    // // Creates buffer clone
    // let mut buffer_clone = buffer.clone();

    // // Removes buffer_size from 100 numbers
    // for n in 0..buffer_size {
    //     numbers_clone.remove(0);
    // }

    // // Prints buffer at start
    // println!("buffer {:#?}", buffer_clone);

    // // REPLACE IN BUFFER

    // // Removes index 0 from buffer 
    // buffer_clone.remove(0);
    // // Inserts index 0 from 100 numbers into index 0 from buffer
    // buffer_clone.insert(0, numbers_clone[0]);
    // // Removes index 0 from 100 numbers
    // numbers_clone.remove(0);

    // println!("Changed buffer: {:#?}", buffer_clone);

    // let mut p = Page::new(numbers_clone[0]);
    // print!("{:#?}", p);
    // p.update_ref();
    // print!("{:#?}", p);
    // let mut r = Report::new();
    // print!("{:#?}", r);
    // let i = 10;
    // r.update_faults(i);
    // print!("{:#?}", r);
    // let t = 2;
    // r.update_hits(t);
    // print!("{:#?}", r);
    // let f = 33;
    // r.update_removes(f);
    // print!("{:#?}", r);
    // r.update_removes(0);
    // print!("{:#?}", r);
    let mut fifo_ref_string = ref_string.clone();
    fifo(fifo_ref_string);



}

pub fn fifo(mut ref_str: Vec<Page>)
{
    let mut fifo_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("frame: {:#?}", frame_buffer);
        println!("Hits:{}, Faults:{}, Removes:{}", fifo_report.hits, fifo_report.faults, fifo_report.removes);
        if frame_buffer.contains(&i){
            i.update_ref();
            fifo_report.update_hits();
        }else if frame_buffer.len() != BUFFER_SIZE {
            frame_buffer.push_front(*i);
            fifo_report.update_faults();
        }else{
            frame_buffer.pop_back();
            frame_buffer.push_front(*i);
            fifo_report.update_removes();
            fifo_report.update_faults();
        }
    }
    println!("Hits:{}, Faults:{}, Misses:{}", fifo_report.hits, fifo_report.faults, fifo_report.removes);

    println!("final: {:#?}", frame_buffer);
    println!("size: {}", ref_str.len());
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