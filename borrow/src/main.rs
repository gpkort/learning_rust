use std::vec;

fn process(input: &mut String) {
    input.push_str("!");
    println!("Greeting: {}", input);
}

fn append_num(vector: &mut Vec<i32>) {
    vector.push(42);
}

fn print_vector(vector: &Vec<i32>) {
    for i in vector {
        println!("{}", i);
    }
}

fn main() {
    let mut s = String::from("hello");
    let mut v = vec![1, 2, 3];

    println!("Borrowing");
    process(&mut s);
    process(&mut s);
    append_num(&mut v);
    print_vector(&v);
    append_num(&mut v); 
    print_vector(&v);

 }