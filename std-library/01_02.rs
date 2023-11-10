use std::collections::{HashMap, LinkedList, VecDeque};

fn main() {
    
    // VECTORS
    // Creating an empty vector of integers
    let mut numbers: Vec<i32> = Vec::new();

    // Adding elements to the vector
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Accessing elements
    println!("Vector: {:?}", numbers);

    // Iterating over elements
    for num in &numbers {
        println!("Number: {}", num);
    }


    // ARRAY 
    // Creating an array of integers
    let numbers: [i32; 3] = [1, 2, 3];

    // Accessing elements
    println!("Array: {:?}", numbers);

    // Iterating over elements
    for num in numbers.iter() {
        println!("Number: {}", num);
    }


    // HASH-MAPS
    // Creating a hash map with keys and values
    let mut user_age: HashMap<&str, u32> = HashMap::new();

    // Inserting key-value pairs
    user_age.insert("Alice", 30);
    user_age.insert("Bob", 25);
    user_age.insert("Charlie", 35);

    // Accessing values by key
    if let Some(age) = user_age.get("Bob") {
        println!("Bob's age: {}", age);
    }

    // Iterating over key-value pairs
    for (name, age) in &user_age {
        println!("{} is {} years old", name, age);
    }


    // lINKED-LIST
    // Creating a linked list of integers
    let mut list: LinkedList<i32> = LinkedList::new();

    // Adding elements to the list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    // Accessing elements
    for num in &list {
        println!("Number: {}", num);
    }


    // QUEUES
    // Creating a double-ended queue of integers
    let mut queue: VecDeque<i32> = VecDeque::new();

    // Enqueuing elements at the back
    queue.push_back(1);
    queue.push_back(2);

    // Dequeuing elements from the front
    if let Some(front) = queue.pop_front() {
        println!("Dequeued: {}", front);
    }

}
