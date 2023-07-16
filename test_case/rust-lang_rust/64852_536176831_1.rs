
fn await_on_apit(x: impl Future<Output = ()>) {                                                     
    x.await;                                                                                        
    //~^ ERROR no field `await` on type                                                             
    //~| NOTE to `.await` a `Future`, switch to Rust 2018                                           
    //~| HELP set `edition = "2018"` in `Cargo.toml`                                                
    //~| NOTE for more on editions, read https://doc.rust-lang.org/edition-guide                    
}
