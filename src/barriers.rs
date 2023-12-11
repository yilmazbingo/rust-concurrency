/*
   A barrier enables multiple threads to synchronize the beginning of some computation. you may consider it like an obstacle or a point in a code which will halt the exectuion of some threads until all the threads have executed the code up to that particular point. when all the threads reaches the point, there will be allowed for further execution of the remaining of the code. 

 */

use std::thread;
use std::sync::{Arc,Mutex,Barrier};
fn mainn(){
    let mut threads=Vec::new();
    // arc will allow multiple threads to use the barrier in a correct way
    // 3 threads will be blocked
    let barrier=Arc::new(Barrier::new(3));
    for i in 0..10{
        let barrier=barrier.clone();
        let t=thread::spawn(move||{
            println!("before wait {}",i);
            // this will block the thread until all the threads have executed upto this particular point.
            barrier.wait();
            // this will execute after all threads are done executing their repsective code up tp the line on 
            println!("After waiting {}",i)
        });
        threads.push(t);
    }
    // all the threads should go to completion
    for t in threads {
        t.join().unwrap();
    }

}

fn main(){
    let mut threads=Vec::new();
    let barrier=Arc::new(Barrier::new(3));
    // since this data will be passed to different threads it is wrapped inside Arc
    let data=Arc::new(vec![vec![1,2,3,4,5],vec![1,2,3,4,5],vec![1,2,3,4,5]]);
    let result=Arc::new(Mutex::new(0));
    for i in 0..3{
        let barrier=barrier.clone();
        let data=data.clone();
        let result=result.clone();
        let t=thread::spawn(move || {
            let x:i32=data[i][0..3].iter().sum();
            *result.lock().unwrap()+=x;
            println!("Thread {} part1 is done",i);
            barrier.wait();
            // then we sum the rest of the array items
            let x:i32=data[i][3..5].iter().sum();
            *result.lock().unwrap()+=x;
            println!("thread {} is complete",i);
        });
        threads.push(t);
    }
    for t in threads{
        t.join().unwrap();
    }
    println!("the final value of the result is {}",*result.lock().unwrap());

}