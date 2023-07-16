rust    
let mut gen = Box::pin(static move |mut arg: &mut i32| {
    loop {
        *arg += 1;
        arg = yield;
    }
});
let mut a = 1;
Pin::new(&mut gen).resume(&mut a);
