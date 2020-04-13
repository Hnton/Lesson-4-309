extern crate rand;
use rand::Rng;
use std::clone::Clone;
use std::collections::VecDeque;
use std::fmt;
use std::time::{Instant};

const BUFFER_SIZE: usize = 3;     // Buffer size (Manually change for now to get 3, 5, 10)



#[derive(Copy, Clone, Debug)]
pub struct Page {
    pub number: i64,
    pub reference: bool,
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
    pub fn update_ref_f(&mut self){
        self.reference = false;
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
    //println!("Original: {:?}", ref_string);
    let _fifo_ref_string = ref_string.clone();
    let _scndchance_ref_string = ref_string.clone();
    let _lru_ref_string = ref_string.clone();
    let _nru_ref_string = ref_string.clone();
    let _clock_ref_string = ref_string.clone();

    let fifo_start = Instant::now();
    fifo(_fifo_ref_string);
    let fifo_duration = fifo_start.elapsed();

    let second_start = Instant::now();
    second_chance(_scndchance_ref_string);
    let second_duration = second_start.elapsed();

    let lru_start = Instant::now();
    lru(_lru_ref_string);
    let lru_duration = lru_start.elapsed();
    
    let nru_start = Instant::now();
    nru(_nru_ref_string);
    let nru_duration = nru_start.elapsed();

    let clock_start = Instant::now();
    clock(_clock_ref_string);
    let clock_duration = clock_start.elapsed();

    println!("\n\nFIFO -          Time elapsed: {:?} ", fifo_duration );
    println!("SECOND CHANCE - Time elapsed: {:?} ", second_duration );
    println!("LRU -           Time elapsed: {:?} ", lru_duration );
    println!("NRU -           Time elapsed: {:?} ", nru_duration );
    println!("CLOCK -         Time elapsed: {:?} ", clock_duration );

    let mut time = Vec::new();

    time.push(fifo_duration);
    time.push(second_duration);
    time.push(lru_duration);
    time.push(nru_duration);
    time.push(clock_duration);
    time.sort();

    println!("\nTime Sorted: {:?}\n\n", time);

}


pub fn fifo(mut ref_str: Vec<Page>)
{

    println!("FIFO Algorithm");


    let mut fifo_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", fifo_report);
        println!("\n{:?}", frame_buffer);
        if frame_buffer.contains(&i){
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
}

// Select one of the pages that has not been used recently
// Reference bit
//      true = Been used recently
//      false = Hasnt been used recently
// If the buffer contains the # then updates the hit and puts ref to true
// If buffer is not full then add # to buffer then update the ref to true
// If last ref in buffer is false, pop back then push # to front then update fault and removes
// If last ref in buffer is true, rotate the last # from the back to the front
pub fn nru(mut ref_str: Vec<Page>)
{

    println!("NRU Algorithm");

    let mut nru_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", nru_report);
        println!("\n{:?}", frame_buffer);
        if frame_buffer.contains(&i){
            nru_report.update_hits();    
            
            let x = frame_buffer.iter().position(|r| r.number == i.number).unwrap();
            frame_buffer[x].update_ref_f();
        }
        else if frame_buffer.len() != BUFFER_SIZE {
            frame_buffer.push_front(*i);
            nru_report.update_faults();

            let x = frame_buffer.iter().position(|r| r.number == i.number).unwrap();
        }
        else if frame_buffer[BUFFER_SIZE - 1].reference == false  {
            frame_buffer.pop_back();
            frame_buffer.push_front(*i);
            nru_report.update_faults();
            nru_report.update_removes();
            
        }
        else if frame_buffer[BUFFER_SIZE - 1].reference == true {
            frame_buffer.rotate_left(BUFFER_SIZE - 1);
        }
    }

}

// If fram buffer contains # then update hit, and update ref to true
// If buffer isnt full add another # to buffer and update faults
// Rotate last # in buffer to front
// Pop last off of buffer
// Push new # onto front
// Update removes and faults
pub fn lru(mut ref_str: Vec<Page>)
{

        println!("LRU Algorithm");


    let mut lru_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", lru_report);
        println!("\n{:?}", frame_buffer);

        if frame_buffer.contains(&i) {
            lru_report.update_hits();
           
            let x = frame_buffer.iter().position(|r| r.number == i.number).unwrap();
            frame_buffer[x].update_ref();

        }
        else if frame_buffer.len() != BUFFER_SIZE {
            frame_buffer.push_front(*i);
            lru_report.update_faults();
        }
        else {
            frame_buffer.rotate_left(BUFFER_SIZE - 1);
            frame_buffer.pop_back();
            frame_buffer.push_front(*i);
            lru_report.update_removes();
            lru_report.update_faults();
        }

            // replace lowest index after "hit" page number with new page number
            // update remove()
    }

}

pub fn second_chance(mut ref_str: Vec<Page>)
{

    println!("Second Chance Algorithm");


    let mut scnd_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", scnd_report);
        println!("\n{:?}", frame_buffer);
        if frame_buffer.contains(&i){
            scnd_report.update_hits();

            let x = frame_buffer.iter().position(|r| r.number == i.number).unwrap();
            frame_buffer[x].update_ref();
        }else if frame_buffer.len() != BUFFER_SIZE {
            frame_buffer.push_front(*i);
            scnd_report.update_faults();
        }else{

            loop {
                if frame_buffer[BUFFER_SIZE - 1].reference == true
                {
                    // println!("\n\nTHIS WAS HIT"); //using for testing
                    // println!("\n{:?}", frame_buffer); //using for testing

                    frame_buffer[BUFFER_SIZE - 1].update_ref_f();
                    frame_buffer.rotate_left(BUFFER_SIZE - 1);
                    // println!("\n{:?}", frame_buffer); //using for testing
                    
                } else if frame_buffer[BUFFER_SIZE - 1].reference == false {
                    frame_buffer.pop_back();
                    frame_buffer.push_front(*i);
                    scnd_report.update_removes();
                    scnd_report.update_faults();
                    break;
                } else {
                    println!("error");
                }  
            }
        }
    }
    println!("\n{}", scnd_report);
}

pub fn clock(mut ref_str: Vec<Page>)
{

    println!("\nClock Algorithm");

    let mut clock_report = Report::new();
    let mut frame_buffer: VecDeque<Page> = VecDeque::with_capacity(BUFFER_SIZE);
    for i in ref_str.iter_mut() {
        println!("\n\n{}", clock_report);
        println!("\n{:?}", frame_buffer);
        if frame_buffer.contains(&i){
            clock_report.update_hits();

            let x = frame_buffer.iter().position(|r| r.number == i.number).unwrap();
            frame_buffer[x].update_ref();
        }else if frame_buffer.len() != BUFFER_SIZE {
            frame_buffer.push_front(*i);
            clock_report.update_faults();
        }else{

            loop {
                if frame_buffer[BUFFER_SIZE - 1].reference == true
                {
                    // println!("\n\nTHIS WAS HIT"); //using for testing
                    // println!("\n{:?}", frame_buffer); //using for testing

                    frame_buffer[BUFFER_SIZE - 1].update_ref_f();
                    frame_buffer.rotate_left(BUFFER_SIZE - 1);
                    // println!("\n{:?}", frame_buffer); //using for testing
                    
                } else if frame_buffer[BUFFER_SIZE - 1].reference == false {
                    frame_buffer.pop_back();
                    frame_buffer.push_front(*i);
                    clock_report.update_removes();
                    clock_report.update_faults();
                    break;
                } else {
                    println!("error");
                }  
            }
        }
    }
    println!("\n{}", clock_report);
}