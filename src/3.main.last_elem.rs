use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
//\x1B is the ESC (ASCII 27) byte. The full string contains two ANSI/VT100 control sequences (CSI sequences):
//    \x1B[2J — ESC [ 2 J : clear the entire screen (erase display).
//    \x1B[1;1H — ESC [ 1 ; 1 H : move the cursor to row 1, column 1 (top-left).

// A data structure to contain the iterator passed in for iteration
struct Progress<Iter> {
    iter: Iter,
    i: usize,
    last_element: Option<usize>
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, last_element: None } // a new instance starting at counter 0
    }
}

// the following method is conditional to the Iter type implementing 
// the ExactSizeIteratori trait. This implementation is constrained
impl<Iter> Progress<Iter> 
where Iter: ExactSizeIterator {
    pub fn with_last_elem(mut self) -> Self {
        self.last_element = Some(self.iter.len());
        self
    }
}



// implementing the trait Iterator for the struct Progress over the type Iter 
// calls for defining the associated type Item and the function next
impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator {
    type Item = Iter::Item; // returns the iterator's item Progress receives 

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR );
        match self.last_element {
            Some(the_last_element) => 
                println!("[{}{}]",
                    "*".repeat(self.i),
                    " ".repeat(the_last_element - self.i)),
            None =>
                println!("{}",  "*".repeat(self.i))
        }
        self.i += 1;
        self.iter.next() 
    }     
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}
 
//Implement the trait ProgressIteratorExt for the Rust struct
//Iter such that it returns an object of type Progress
impl<Iter> ProgressIteratorExt for Iter 
where Iter: Iterator {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}



fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}


fn main() {
    // The following API call is a syntax error
//    for n in (0 .. ).progress().with_last_elem() {
//        expensive_calculation(&n);
//    }
    let v = vec![1,2,3,4,5];
    for n in v.iter().progress().with_last_elem() {
        expensive_calculation(n);
    }
}
