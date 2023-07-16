rust
struct State {
    wrapper: Wrapper,
    counter: u64,
}

// Wrapper to make sure this isn't Clone / Copy.
struct DataType(u64);

struct FrameData<'sim> {
    data: &'sim DataType
}

mod inner {
    use std::cell::RefCell;
    use super::*;

    // This wrapper is needed so that nothing else can call `borrow` or `borrow_mut`.
    pub struct Wrapper {
        data: RefCell<DataType>
    }

    impl Wrapper {
        pub fn new() -> Self {
            Self {
                data: RefCell::new(DataType(0))
            }
        }

        pub fn simulate(&mut self, new_value: DataType) {
            *self.data.borrow_mut() = new_value;
        }

        pub fn wrapper_frame_data<'frame, 'sim: 'frame>(&'sim self, frame_data: &mut Vec<FrameData<'frame>>) {
            let guard = self.data.borrow();
            let data: &DataType = &*guard;

            // Transmute the data to the `sim lifetime, because we know that the state can't be modified during that lifetime.
            let data: &'sim DataType = unsafe { std::mem::transmute(data) };
            frame_data.push(FrameData {
                data
            });
        }
    }
}

use inner::Wrapper;

fn simulate(state: &mut State) {
    state.counter += 1;

    // This is the only place where the `wrapper` could be modified -- we need a &mut reference.
    state.wrapper.simulate(DataType(state.counter));
}

fn state_frame_data<'frame, 'sim: 'frame>(state: &'sim State, frame_data: &mut Vec<FrameData<'frame>>) {
    // `wrapper` cannot be mutated or dropped while `&'sim State` lives.
    state.wrapper.wrapper_frame_data(frame_data);
}

fn render(state: &State) {
    let mut frame_data = Vec::new(); // <-- This definitely lives less time than `State`.
    state_frame_data(state, &mut frame_data);
    for data in frame_data {
        println!("{}", data.data.0);
    }
}

fn main() {
    let mut state = State {
        wrapper: Wrapper::new(),
        counter: 0,
    };

   loop {
        simulate(&mut state);
        render(&state);
    }
}
