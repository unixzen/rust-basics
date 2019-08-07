use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..30 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    handle.join();

    for i in 1..15 {
        println!("hi number {} from the main thread!", i);
    }

}

