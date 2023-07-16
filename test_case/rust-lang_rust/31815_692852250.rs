diff
fn call<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

fn main() {
-     let _: (&[i32],) = call(|| (&[],));
+     let _: (&[i32; 0],) = call(|| (&[],));
}
