/*
      with share state, we have some piece of data residing inside the memory that multiple threads can read and write to at the same time

      - MUTEX = mutual exclusion means that you have some piece of data and ONLY ONE THREAD can access at any given time. Mutexes uses locking system. when a thread wants to access
      to piece of data behind a mutex, it will signal that it wants to access to that data and acquire a LOCK on the mutex. 
      - The LOCK is a type of data structure that keeps track of which thread has exclusive access to a piece of data. Once a thread has acquired a lock on a particular piece of data,
      no other thread can access that data. oNce the thread is done with that piece of data, it can unlock the data and allow other threads to have access to that particular data
      
      - Mutex does have a reputation for being hard to manage because you have to remember two rules
        1- You have to acquire a lock before you have access to the data
        2- you have to release that lock once you are done with the data
 */

use std::sync::Mutex;
fn mainn(){
    let m=Mutex::new(5);
    // creating an inner scope
    {
        // if lock fails, it will panic
        // after lock().unwrap() we get a mutex guard smart pointer whose deref trait points to the inner data of the mutex
        // mutex guard also implements drop trait
        let mut num=m.lock().unwrap();
        // * dereference operator
        *num=10;
        // when mutex guard goes out of scope, then it releases the lock to the data. releasing the lock will be automatically for you
    }
    println!(" m={:?}",m)
}

fn main(){
    let m=Mutex::new(5);
    let mut num=m.lock().unwrap();
    *num=12;
    drop(num);// this will automatically unlock the mutex
    // this blocks the thread
    let mut num1=m.lock().unwrap();
    // a deadlock is a situation in which threads sharing the same resource are effectively preventing each other from accessing the resource, resulting in haltinng or blocking of both
    //correct way was to mutate the vairable and then unlock it 
}