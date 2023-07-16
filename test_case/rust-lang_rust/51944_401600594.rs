rust
let pinbox = PinBox::new(/*...*/);
let obj1 = FutureObj::new(pinbox.as_pin_mut());
let obj2 = FutureObj::new(pinbox);
drop(obj2); // this will drop the future
executor.spawn(obj1); // I think it's a use-after-free
