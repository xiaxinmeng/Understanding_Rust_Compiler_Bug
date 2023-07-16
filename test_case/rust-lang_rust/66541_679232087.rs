rust
trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for &[u8] {
    fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self) }
    }
}

fn foo(buf: &[u8]) -> &str {
    unsafe { str::from_utf8_unchecked(buf) }
}

// Error: cannot return value referencing temporary value
fn bar(buf: &[u8]) -> &str {
    return buf.borrow().as_str();
}

// Okay
fn baz(buf: &[u8]) -> &str {
    return foo(buf);
}

// Error: cannot return value referencing function parameter `buf`
fn qux(buf: &[u8]) -> &str {
    return AsStr::as_str(&buf);
}

