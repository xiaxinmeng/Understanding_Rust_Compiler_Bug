rust
struct HasDrop<T>(T);
impl<T> Drop for HasDrop<T> {
    fn drop(&mut self) {}
}

#[allow(unused_assignments)]
#[allow(unused_variables)]
fn foo() {
    let mut first: u8 = 0;
    let mut has_drop = Some(HasDrop(&first));

    loop {
        match true {
            true => {
                has_drop = Some(HasDrop(&first));
            }
            false => {
                drop(has_drop);
                has_drop = None;
                let reborrow = &mut first;
            }
        }
        
    }
}

fn main() {}
