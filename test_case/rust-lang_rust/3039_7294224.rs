
fn key(+_x: @ticky_tacky) { }

class ticky_tacky {
    let data: uint;
    new(data: uint) { self.data = data; }
    drop unsafe {
        do task::local_data_modify(key) |box| {
            alt box {
                some(x) { #error["My own data: %u", x.data]; }
                none    { fail ~"????" }
            }
            box
        }
    }
}

fn main() {
    unsafe { task::local_data_set(key, @ticky_tacky(42)); }
    fail;
}
