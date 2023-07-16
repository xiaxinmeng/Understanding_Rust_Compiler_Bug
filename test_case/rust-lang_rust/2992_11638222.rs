
// rustc --test funops.rs
extern mod std;

pub type Parser<T: Copy Durable> = fn@ (State) -> Status<T>;
pub struct State {file: @~str, text: @[char], index: uint}
pub type Status<T: Copy Durable> = Result<Succeeded<T>, Failed>;
pub struct Succeeded<T: Copy Durable> {new_state: State, value: T}
pub struct Failed {old_state: State, err_state: State, mesg: @~str}

// Used to create a function that will parse self or rhs.
pub impl<T: Copy Durable> Parser<T> : ops::BitOr<Parser<T>, Parser<T>>
{
    pure fn bitor(&self, _rhs: &Parser<T>) -> Parser<T>
    {
        |input: State|
        {
            match (*self)(input)
            {
                result::Ok(ref pass1) => 
                {
                    // This is not what the real code does...
                    result::Ok(Succeeded {new_state: pass1.new_state, value: pass1.value})
                }
                result::Err(ref failure1) =>
                {
                    result::Err(Failed {old_state: input, ..*failure1})
                }
            }
        }
    }
}

// funops.rs:17:20: 17:24 error: value may contain borrowed pointers
// funops.rs:17             match (*self)(input)
//                                  ^~~~
