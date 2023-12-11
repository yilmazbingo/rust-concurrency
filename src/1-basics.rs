/*
 Concurrency is also referred as multithreading
 In most os, executed programs code is run in a process and os manages multiple processes at once. within your program, you can also have independent parts that run simultaneously
 this feature that runs these independend parts is called "THREADS"
 bY using threads we can have concurrent code. 
*/

use std::thread;
use std::time::Duration;

// https://stackoverflow.com/questions/75272906/why-print-statements-inside-rust-thread-do-not-run
// each program by default has one thread, which is the main thread
fn main() {
    // println!("First Hello, world!");
    // println!("Second Hello, world second!");
    // println!("Hello, world third!");
    // concurrency starts here. after that print inside thread and in main will execute in parallel. order is not deterministic. scheduling order is handled by the underlying system
    // each program by default has one thread, which is the main thread
    /*
       sometimes thread is executed partially and then switch to the main, and at other times the main is executed first and then a chance given to the thread. And at other times multiple switches may take place between the main and the created threat. as explained before, this is because the thread scheduling are being handled by the OS. 
       some of the print statements inside the thread may not execute  because main thread will always complete execution before the termination of the program while the remaining 
       of thread may or may not. Once the main thread of the program completes, all spawned threads are shut down whether or not they have finished their respective execution.
    */
    /*
       the concurrency is about multiple tasks which starts run and complete in overlapping time periods in no specific order.
       Parallellism is about multiple tasks or sub tasks of the same task that literally run at the same time on a hardware with multiple computing resoruces like multi-core processors. 
       Concurrency is like juggling multiple tasks. You're managing different tasks at the same time, but you might not be doing them all simultaneously. For example, you might start one task, switch to another, then come back to the first one. It's about managing tasks and making progress on each, but they might not all be happening at the exact same moment.

Parallelism is like having multiple hands to do things. When you have parallelism, you're actually doing multiple tasks truly at the same time. For instance, if you have a four-core processor, it's like having four people working independently at the same time, each handling a different task. They're all happening simultaneously because you have the resources to do so.

     threads are often associated with problems such as race conditions where threads are accessing data or resource in an inconsistent order and deadlocks where two threads are waitin for each other to finish using a resource , preventing both threads from continuing.

     */
    thread::spawn( || {
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
        println!("Hello 1, world  thread!");
        println!("Hello 2, world  thread!");
        println!("Hello 3, world  thread!");
    });
    // let us learn how to ensure that the spawn thread runs to the completion before the end of the main program and do not end prematurely. 
    // the joinHandle is the returning value of a thread spawn function which we can store in a variable
    let t=thread::spawn(||{
        println!("joinHandle 1, world  thread!");
        println!("joinHandle 2, world  thread!");
        println!("joinHandle 3, world  thread!");
        println!("joinHandle 1, world  thread!");
        println!("joinHandle 2, world  thread!");
        println!("joinHandle 3, world  thread!");
        println!("joinHandle 1, world  thread!");
        println!("joinHandle 2, world  thread!");
        println!("joinHandle 3, world  thread!");
    });
    t.join();
    // this will block the main thread for one milisecond, thereby giving a slightly more chance
    // thread::sleep(Duration::from_millis(1));
    println!("Hello 1, world main function!");
    println!("Hello 2, world main function!");
    println!("Hello 1, world main function!");
    println!("Hello 2, world main function!");
    println!("Hello 1, world main function!");
    println!("Hello 2, world main function!");
}
