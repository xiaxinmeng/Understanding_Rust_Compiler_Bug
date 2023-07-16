
let mut x = Some(1);
let res = {
    let mut flag = false;
    let mut v = if let Some(&ref p) = x.as_ref() {
        // do something with p
        println!("{}", p);
        p * 3
    } else {
        flag = true;
        unsafe { std::mem::uninitialized() }
    };
    if flag {
        x = Some(2);
        v = 7;
    }
   v
};
