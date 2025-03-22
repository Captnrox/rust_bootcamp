use std::cell::RefCell;
use std::thread;

thread_local! {
    static FOO: RefCell<u32> = RefCell::new(0);
}

fn main() {
    let handle = thread::spawn(|| {
        FOO.with(|foo| {
            *foo.borrow_mut() = 1;
            println!("Thread-local value in the spawned thread: {}", foo.borrow());
        });
    });

    let handle2 = thread::spawn(|| {
        FOO.with(|foo| {
            let mut val = foo.borrow_mut();
            *val += 5;
            *val = *val * 2 - 3;
            drop(val);
            println!("Thread-local value in the spawned thread: {}", foo.borrow());
        });
    });

    FOO.with(|foo| {
        *foo.borrow_mut() = 2;
        println!("Thread-local value in the main thread: {}", foo.borrow());
    });

    handle.join().unwrap(); 

    handle2.join().unwrap();
}