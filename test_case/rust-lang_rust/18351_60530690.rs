 rust
//standard library imports
use std::rt::thread::Thread;
use std::sync::{ Arc, Mutex};

fn main(){
    //make mutex
    let done = Arc::new( Mutex::new( false));

    //spawn producer
    println!("starting producer");
    let mut prod_done = done.clone();
    let producer = std::task::try_future(
        proc(){ *( prod_done.lock()) = true;});

    //spawn consumer
    println!("starting consumer");
    let consumer = std::task::try_future(
        proc(){
            while ! *( done.lock()) {}});

    //finish up
    println!("joining threads");
    producer.unwrap();
    consumer.unwrap();
    println!("threads joined");
}
