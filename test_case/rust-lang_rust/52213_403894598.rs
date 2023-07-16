rust
enum Inner {
    Stack {
        data: [u8;23]
    },
    Heap {
        data: Box<[u8]>
    }
}

struct SmallString {
    len: usize,
    inner: Inner
}

impl SmallString {
    fn push_str(&mut self, item: &str) {
        match (&mut self.inner, self.len + item.len()) {
            (Inner::Heap { data }, x) => {
                println!("{}", data.len());
                if x > data.len() {
                    self.grow();
                    // data is now garbage pointer
                }
                println!("{:?}", data);
            },
            _ => ()
        }
    }
    fn grow(&mut self){
        // Invalidate borrowed Heap.data
        self.inner = Inner::Stack { data: [1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1,0,1] };
    }
}

fn main (){
    let slice = "this is gonna go bad".to_owned().into_bytes().into_boxed_slice();
    let mut ss = SmallString { len: slice.len(), inner: Inner::Heap { data: slice } };
    ss.push_str(" right now");
}
