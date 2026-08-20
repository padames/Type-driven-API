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


// The ProgressDisplay trait will be used to define how the Progress bar
// behaves when the method display is called on a type that implements it.
trait ProgressDisplay : Sized {
    fn display<Iter>(&self, progress: &Progress<Iter,Self>);
}

// here is how display works for instances of the type Unbounded
impl ProgressDisplay for Unbounded {
    // display can only use the current state of the bar, namely.
    // the count element i to represent its advance
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}",  "*".repeat(progress.i))
    }
}

// and here is how display works for instances of the type Bounded
impl ProgressDisplay for Bounded {
    // display can use the left and right elements of the book ends
    // that a Bounded object has. It can also compute the difference
    // between the number of elements to iterate over and the current
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}{}{}",
            self.book_ends.0, //an instance of Bounded has book_ends
            "*".repeat(progress.i),
            " ".repeat(self.num_elements - progress.i),
            self.book_ends.1)
    }
}

// A data structure to contain the iterator passed in for iteration
struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound_type: Bound,
}

// An associated function to create progress bar instances
// The initial state is always Unbounded
impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound_type: Unbounded }// the state starts at 0
    }
}

// The method with_last_elem is defined for cases where the parameter type Iter
// implements the ExactSizeIterator trait. This is a trigger for a change of 
// type state of the progress bar. It is specified as the change of the bounded
// type to type Bounded with default book ends and a known number of elements.
impl<Iter> Progress<Iter, Unbounded> 
where Iter: ExactSizeIterator {
    pub fn with_last_elem(self) -> Progress<Iter, Bounded> {
        let bound_type: Bounded = Bounded {
            num_elements: self.iter.len(),
            book_ends: ( '[', '}')
        };
        // the following is the new state of the Progress bar
        Progress { i: self.i, iter: self.iter, bound_type: bound_type }
    }
}

// A parameterized Progress method to assign custom book ends
// Note: the type bound called Bounded constraints its application to 
// when the type state is an instance of Bounded
impl<Iter> Progress<Iter, Bounded> {
    pub fn with_book_ends(mut self, book_ends: (char, char)) -> Self {
        self.bound_type.book_ends = book_ends;
        self
    }
}

// Implementing the trait Iterator for the type Progress when the associated type
// Iter defines the trait Iterator. 
// calls for defining the associated type Item and the function next
impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item; // returns the iterator's item Progress receives 

    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR );
        self.bound_type.display(&self);
        self.i += 1; // this is the state of the progress bar
        self.iter.next() 
    }     
}

// This trait is used to create the progress bar with its initial type state
trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}
 
// Implements ProgressIteratorExt for the Rust type Iter such that it returns
// a new Progress bar
impl<Iter> ProgressIteratorExt for Iter 
where Iter: Iterator {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

// simulating useful work
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}


fn main() {
    let a_book_end: (char, char) = ('<', '>');
    // The following API call will produce errors because progress is Unbound by default 
//    for n in (0 .. ).progress().with_book_ends(book_end) {
//       expensive_calculation(&n);
//    }
    let v = vec![1,2,3,4,5];
    for n in v.iter().progress().with_last_elem().with_book_ends(a_book_end) {
        expensive_calculation(n);
    }
}
