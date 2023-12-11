use std::thread;


fn mainn(){
   let mut thread_vec=vec![];
   /* -  variable i is part of main thread, not part of the thread that we are creating. Moreover the closure inside the thread takes no arguments, which means that 
         we are not using any data from the main thread. to use the data from the main thread in spawn thread, the spawn thread closure must capture the values it needs. 
    since we are not sure that the thread will live long enough as the main, therefore to avoid possible lifetime issue, rust demands that the value may be moved inside the closure
    which resides inside the thread
   */
   for i in 0..10 {
    // we transfer the i inside the thread. "move" is used to take ownership of the values that the closure uses from. i is transferred to inside the thread
    thread_vec.push(thread::spawn(move ||{
        // some of the threads did not get a chance to be executed. To make sure that all the threads get a chance to execute we use `join`
        println!("thread number {}",i)
    }))
   }
   // since there are 10 thread, we need 10 join_handle
   for i in thread_vec{
    i.join();
   }
}

fn main(){
    let v=vec![1,2,3];
    let x=5;
    // IF I WANT TO drop x, i need to set its boundary  let x=5;
    
    let handle=thread::spawn(move ||{
         println!("here is a vecto :{:?}",v);
         println!("here is the primitive value {:?}",x)
    });
    // If I had a primitive value passed to the handle, this code would be fine,becuase primitive values implement Copy trait and drop does not work for the copy types
    drop(v);
    println!("the variable x is still alive If i drop x {}",x);
    println!("the variable v is not alive If i drop v {}",v);

    handle.join();
}

fn main2(){
    let x=5;
    let y=3;
    {x};
    let z=x;
}