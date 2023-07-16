
ice.rs:11:15: 11:20 error: cannot implement a destructor on a structure that does not satisfy Send
ice.rs:11 impl Drop for Boxer {
                        ^~~~~
ice.rs:11:15: 11:20 note: use "#[unsafe_destructor]" on the implementation to force the compiler to allow this
ice.rs:11 impl Drop for Boxer {
                        ^~~~~
error: aborting due to previous error
