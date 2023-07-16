
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;

fn foo(_: impl AsRef<Path>) {}
fn bar(_: impl AsRef<i32>) {}

fn main() {
    foo("Hi!");
    foo("How are you?".to_string());
    foo(&&&&&&("I'm fine.".to_string()));
    foo(&&&&&&&&&&&&&&&&&&&"Still doing well here.");
    //bar(5); // ouch!
    //bar(&5); // hmmm!?
    bar(Cow::Borrowed(&5)); // phew!
    foo(Cow::Borrowed(OsStr::new("Okay, let me help!")));
    foo(&&&&&&&&Cow::Borrowed(OsStr::new("This rocks!!")));
    //foo(&&&&&&&&Cow::Borrowed("BANG!!"));
}
