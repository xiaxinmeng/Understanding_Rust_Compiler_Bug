
src/glib/traits.rs:46:5: 46:48 help: consider using an explicit lifetime parameter as shown: fn connect<'a>(&self, signal: Box<T>)
src/glib/traits.rs:46     fn connect<'a>(&self, signal: Box<T>) -> () {
                          ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
src/glib/traits.rs:49:22: 49:47 error: mismatched types: expected `gtk::signals::Signal<'a>`, found `gtk::signals::Signal<'a>` (lifetime mismatch)
src/glib/traits.rs:49         let signal = signal as Box<Signal<'a>>;
