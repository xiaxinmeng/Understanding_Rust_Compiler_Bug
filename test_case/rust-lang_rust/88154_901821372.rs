diff
#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let mut generator = || {
        yield 1;
        return "foo"
    };

-    match Pin::new(&mut generator).resume(()) {
-        GeneratorState::Yielded(1) => {}
-        _ => panic!("unexpected return from resume"),
-    }
+    assert!(Pin::new(&mut generator).resume(()).is_yielded());
-    match Pin::new(&mut generator).resume(()) {
-        GeneratorState::Complete("foo") => {}
-        _ => panic!("unexpected return from resume"),
-    }
+    assert!(Pin::new(&mut generator).resume(()).is_complete());
}
