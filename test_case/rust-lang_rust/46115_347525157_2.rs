rust
#[no_mangle]
pub fn say_hello() -> &'static str {
	"hello, world! hello, rust wasm!"
}
