use std::sync::Mutex;
use std::thread;
/*  we cannot send REFERENCE COUNTING SMART POINTERS (RC) safely between the threads. Rc gives us the functionality that we want, but unfortunately it is not thread safe. this is 
because if manages reference counting for keepING tracks of owenrs. each time you call .clone, it increases count, if you call drop, it substracts the count. But it does not use any
concurrency primitive to make sure that changes to the count cannot be interrupted by another thread and therefore we will always have a correct value. this means count variable ITSLEF
is not designed from the perspective of concurrency. 

*/
/*
    We need somehting like RC but thread safe
   Atomic Referece counting ARC are concurrency primitives which faciliates a shared memory communication between threads and are the building blocks of other concurrent types. 
 */
use std::rc::Rc;
use std::sync::Arc;
fn mainn(){
    /* counter is immutable but we are able to get mutable reference because mutex uses interior mutability. this is similar to Refcell.
       Mutex smart pointers allows us to mutate a value inside the Arc smart pointer
      
     */
    let counter=Arc::new(Mutex::new(0));
    // we need this for calling the join function which will ensure that all the threads goes to completion
    let mut handles=vec![];
    // what we want here is to allow multiple owners of the counter so that each one is able to have ownership
    for _ in 0..10{
        let counter=Arc::clone(&counter);
        let handle=thread::spawn(move ||{
            let mut num=counter.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("Result :{}",*counter.lock().unwrap());
}


// how to use Arc for cloning types which does not implement Clone
/*  all thought, this does not implement the Clone, however we have clone of it inside each thread.
    In general, Arc can be used to clone types which are not clone and does not implement the clone. 
    we should have use Mutex but since we are only reading the data, it is ok

*/
struct MyString(String);

impl MyString{
    fn new (s:&str)->MyString{
        MyString(s.to_string())
    }
}
fn main(){
    let mut threads=Vec::new();
    let name=Arc::new(MyString::new("rust"));
    for i in 0..5{
        let some_str=name.clone();
        let t=thread::spawn(move||{
            println!("hello {:?} count {}",some_str.0,i)
        });
        threads.push(t);
    }
    for t in threads{
        t.join().unwrap();
    }
}