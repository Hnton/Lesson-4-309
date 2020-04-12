extern crate rand;
use rand::Rng;
use std::clone::Clone;
use std::collections::VecDeque;
use std::fmt;
const BUFFER_SIZE: usize = 3;     // Buffer size (Manually change for now to get 3, 5, 10)



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

impl fmt::Display for Page{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number:{}     |      Reference:{}", self.number, self.reference)
    }
}

#[derive(Debug)]
pub struct Report {
    pub hits: i64,
    pub removes: i64,
    pub faults: i64
}

impl fmt::Display for Report{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hits:{} | Faults:{} | Removes:{}", self.hits, self.faults, self.removes)
    }
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
    // Random number generator
    let mut random_number = rand::thread_rng();

    // Creating Vec of 100 numbers
    let mut ref_string = Vec::<Page>::new();
    
    // Pushing 100 random numbers from 0 - 19 into Vec
    for _ in 0..100 {
        ref_string.push(Page::new(random_number.gen_range(0,20)));
    }
    // Prints original 100 numbers
    println!("Original: {:?}", ref_string);
    let mut fifo_ref_string = ref_string.clone();
    fifo(fifo_ref_string);



}

pub fn fifo(mut ref_str: Vec<Page>)
{
    let mut fifo_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", fifo_report);
        println!("\n{:?}", frame_buffer);
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
    println!("\n{}", fifo_report);

    println!("{:?}", frame_buffer);}

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