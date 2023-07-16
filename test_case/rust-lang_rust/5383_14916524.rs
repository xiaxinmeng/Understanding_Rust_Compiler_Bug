
fn bogus_key(_v: @uint) {}


pub fn cause_bug()  {
    unsafe {
        task::local_data::local_data_get(bogus_key);
    }
}

fn main () {
    cause_bug();
    fail!();
}
