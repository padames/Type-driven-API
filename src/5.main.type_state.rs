use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";
//\x1B is the ESC (ASCII 27) byte. The full string contains two ANSI/VT100 control sequences (CSI sequences):
//    \x1B[2J — ESC [ 2 J : clear the entire screen (erase display).
//    \x1B[1;1H — ESC [ 1 ; 1 H : move the cursor to row 1, column 1 (top-left).

// part 5: Introduce type state to indicate when the Progress
//         is of a type that expects book ends aor not

struct Unbounded;
struct Bounded {
    num_elements: usize,
    book_ends: (char, char)
}

// A data structure to contain the iterator passed in for iteration
struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound_type: Bound,
}

// the following trait ties the two new types to the Progress bar
trait ProgressDisplay : Sized {
    fn display<Iter>(&self, progress: &Progress<Iter,Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}",  "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}{}{}",
            self.book_ends.0,
            "*".repeat(progress.i),
            " ".repeat(self.num_elements - progress.i),
            self.book_ends.1)
    }
}


impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound_type: Unbounded }// a new instance starting at counter 0
    }
}

// the following method is conditional to the Iter type implementing 
// the ExactSizeIterator trait. This implementation is constrained
impl<Iter> Progress<Iter, Unbounded> 
where Iter: ExactSizeIterator {
    pub fn with_last_elem(self) -> Progress<Iter, Bounded> {
        let boundType: Bounded = Bounded {
            num_elements: self.iter.len(),
            book_ends: ( '[', '}')
        };
        // the following is the new state of the Progress bar
        Progress { i: self.i, iter: self.iter, bound_type: boundType }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_book_ends(mut self, bookEnds: (char, char)) -> Self {
        self.bound_type.book_ends = bookEnds;
        self
    }
}

// implementing the trait Iterator for the struct Progress over the type Iter 
// calls for defining the associated type Item and the function next
impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item; // returns the iterator's item Progress receives 

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR );
        self.bound_type.display(&self);
        self.i += 1;
        self.iter.next() 
    }     
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}
 
//Implement the trait ProgressIteratorExt for the Rust struct
//Iter such that it returns an object of type Progress
impl<Iter> ProgressIteratorExt for Iter 
where Iter: Iterator {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}



fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}


fn main() {
    let book_end: (char, char) = ('#', '#');
    // The following API call will produce errors because progress is Unbound by default 
//    for n in (0 .. ).progress().with_book_ends(book_end) {
//       expensive_calculation(&n);
//    }
    let v = vec![1,2,3,4,5];
    for n in v.iter().progress().with_last_elem().with_book_ends(book_end) {
        expensive_calculation(n);
    }
}
