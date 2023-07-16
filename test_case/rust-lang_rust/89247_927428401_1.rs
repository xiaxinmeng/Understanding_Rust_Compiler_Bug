rust
const fn some_const_fn() {
    if let Some(ret) = call_if_rt(|| {
		// runtime part
	}) {
		return ret;
	}
	// const part
}
