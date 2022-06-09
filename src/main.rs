// JavaScript's ES6 Array Methods are extremely helpful and widely used
// Below will demonstrate how to accomplish these in Rust
// The methods targeted are: forEach, map, filter, find, findIndex, some, every, and reduce

fn main() {
    // feel free to  test the code in the main function!
    // forEach is demonstrated below

    // test vector result
    let es6 = SampleArray::new();
    es6.map().iter().for_each(|x| { println!("{}", x)});

    // test optional result
    let optional_result = es6.findIndex();

    if let Some(result) = optional_result {
        println!("result found was {}", result);
    } else {
        println!("No result found");
    }

    // test boolean result
    let result = es6.every();
    println!("Result found was: {}", result);

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

    // find
    pub fn find(&self) -> Option<&i8> {
        self.numbers.iter().find(|num| {
            **num == 3
        })
    }

    // findIndex
    pub fn findIndex(&self) -> Option<usize> {
        self.numbers.iter().position(|num| {
            *num == 3
        })
    }

    // some
    pub fn some(&self) -> bool {
        self.numbers.iter().any(|num| {
            *num == 3
        })
    }

    // every
    pub fn every(&self) -> bool {
        self.numbers.iter().all(|num| {
            *num < 10
        })
    }
}