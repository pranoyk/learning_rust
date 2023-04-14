use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    /*
    In the above implementation, the closure borrows the list variable immutably.
    This means that the closure can only read the list variable, but cannot modify it.
    Also, the closure borrows the list variable for the entire duration of its execution.
    This means that the closure cannot outlive the list variable.
     */
    println!("----------------------------------");

    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list2);
    /*
    In the above implementation, the closure borrows the list variable mutably.
    This mutable borrow occurs because we are adding an element to the list2 variable.
    This means that the closure can modify the list variable, but cannot outlive it.
    Hence we cannot even add a print statement in between the closure definition and its call.
     */
    println!("----------------------------------");

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);
    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();
    /*
    In the above implementation, the closure moves the list3 variable using move keyword into new thread.
    The new thread might outlive the main thread, in that case the borrow becomes invalid.
    Hence, the closure moves the list3 variable into the new thread.
     */
}