/*
   Scoped threads allows the threads to borrow local variables with lifetime more effectively. The problem with threads is that when using threads, we dont know exactly when the
     closure we give to thread spawn will run or execute, meaning it is hard to determine when variables borrowed by threads will be dropped. 
 */

use std::thread;

fn calc_value(num:f32)->f32{
    num
}
fn process_list(list: Vec<f32>) -> Vec<f32> { // Block F
    let chunk_size = 100;
    let output_list = vec![0.0f32;list.len()];

    thread::scope(|s| { // Block S
        (0..list.len()).collect::<Vec<_>>().chunks(chunk_size).for_each(|chunk| { // Block T
            s.spawn(|| {
                chunk.into_iter().for_each(|&idx| {
                    let value = calc_value(list[idx]);
                    unsafe {
                        let out = (output_list.as_ptr() as *mut f32).offset(idx as isize);
                        *out = value;
                    }
                });
            });
        });
    });
    output_list
}
// ownership is stayed in main program
fn main(){
    let mut vec=vec![1,2,3];
    let mut x=0;
    //thread::scope allows us to define a scope in which we can spawn threads. All the threads that resides inside the scope will remain in that scope and will complete its exectuion in the scope
    // we did not move ownership of the variable inside the thread. compiler did not complain because the lifetime of the vars are fine
    // thread scope enforces the threads to complete within this scope and this ensures that this thread will complete before the end of the main thread. 
    // the thread is not allowed to live beyond the scope in which it is defined in this case. 
    thread::scope(|some_scope|{
        some_scope.spawn(||{
            println!("I am first thread in the scope");
            println!("{:?}",vec)
        });

        some_scope.spawn(||{
            println!("I am second thread in the scoped");
            x+=5;
            // we can have 1 mutable scope 
            // vec.push(33); in above thread it is borrowed as immutable. mutable and immutable borrow cannot coexist
            println!("we can have multiple immutable references {:?}",vec);
        });
    // those threads are automatically being joined at the end of the scope
    });

    println!("the threads are now complete");
    vec.push(12);
    println!("x:{:?}, vec:{:?}",x,vec);
}