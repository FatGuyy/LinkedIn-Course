// Basic server in std 


use std::thread;

fn main() {
    // Spawn two threads to run concurrently
    let handle1 = thread::spawn(|| { // make new thread
        for i in 1..=5 {
            println!("Thread 1: Count {}", i);
        }
    });

    let handle2 = thread::spawn(|| { // make another new thread
        for i in 1..=5 {
            println!("Thread 2: Count {}", i);
        }
    });

    // Wait for both threads to finish
    handle1.join().unwrap(); // Use join() to wait for thread to finish, before continuing in the main thread
    handle2.join().unwrap(); 

    println!("Main thread: All threads have finished.");
}
