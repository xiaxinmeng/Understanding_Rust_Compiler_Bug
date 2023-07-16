rust
use std::{collections::HashMap, thread, time::Duration};

#[derive(Debug, PartialEq)]
enum Example {
    Vec(Vec<u8>),
    Test(u32),
    None
}

fn to_static_str<S: AsRef<str>>(str: S) -> &'static str {
    let boxed = str.as_ref().to_owned().into_boxed_str();
    
    // Use Box::into_raw we can reclaim ownership with from_raw
    unsafe { &*Box::into_raw(boxed) }
}

unsafe fn free_static_str(str: &'static str) {
    // Take back ownership using unsafe code
    let boxed = Box::from_raw(str as *const str as *mut str);
    
    // Not necessary since it would get dropped anyway at the end of the function,
    // but it makes the intent more explicit
    std::mem::drop(boxed);
}

fn test_hashmap_static_str(i: i32) {
    let mut map: HashMap<&'static str, Example> = HashMap::new();
    let mut vec_keys_hashmap = Vec::new();

    println!("load memory");
    for num in 0..100_000 {
        
        let str_num = format!("{}_test_{}", num, i);
        let key_hashmap = to_static_str(str_num);

        map.insert(key_hashmap, Example::Vec(vec![0; 100_000]));
        
        vec_keys_hashmap.push(key_hashmap);
    }
    thread::sleep(Duration::from_secs(5));

    println!("clear memory");
    // map.clear();
    // map.shrink_to_fit();
    // drop(map);

    for key_hashmap in vec_keys_hashmap {
        unsafe { free_static_str(key_hashmap); }
    }
}

fn main() {
    for i in 0..5 {
        test_hashmap_static_str(i);
        thread::sleep(Duration::from_secs(5));
    }
    
    println!("END");
    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
