rust
use std::mem::size_of;
use std::sync::Arc;
use std::sync::Mutex;

#[inline(never)]
#[no_mangle]
pub fn thread1(v: &mut bool) {
    *v = true;
}

#[inline(never)]
#[no_mangle]
pub fn thread2(v: &Option<Mutex<bool>>) -> usize {
    match *v {
        None => 0,
        Some(_) => 1,
    }
}

fn main() {
    // Verify that discriminant elision takes place.
    assert_eq!(
        size_of::<Mutex<bool>>(),
        size_of::<Option<Mutex<bool>>>(),
    );

    let a = Arc::new(Some(Mutex::new(false)));
    let b = a.clone();

    let t = std::thread::spawn(move || {
        thread1(&mut (*a).as_ref().unwrap().lock().unwrap())
    });

    println!("{}", thread2(&b));
    t.join().unwrap();
}
