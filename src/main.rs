// JavaScript's ES6 Array Methods are extremely helpful and widely used
// Below will demonstrate how to accomplish these in Rust
// The methods targeted are: forEach, map, filter, find, findIndex, some, every, and reduce

fn main() {
    // feel free to  test the code in the main function!
    // forEach is demonstrated below
    let es6 = SampleArray::new();
    es6.filter().iter().for_each(|x| { println!("{}", x)});

    println!("Hello, ES6 in Rust!");
}

// we will store a vector of i8 integers in a struct
// and implement various ES6 methods on them
// similar to ES6, the resulting vector does not mutate the original

// for all of these, the iter() method is used to create an iterator
// which then gives access to these various methods
// finally, collect() is used at the end to gather the data into a new vector

struct SampleArray {
    numbers: Vec<i8>
}

impl SampleArray {
    // initializer
    pub fn new() -> Self {
        Self {
            numbers: vec![1,2,3,4,5]
        }
    }

    // map
    pub fn map(&self) -> Vec<i8> {
        self.numbers.iter().map(|num| {
            num * 2
        }).collect()
    }

    // filter
    pub fn filter(&self) -> Vec<&i8> {
        self.numbers.iter().filter(|num| {
            *num % 2 == 0
        }).collect()
    }


}