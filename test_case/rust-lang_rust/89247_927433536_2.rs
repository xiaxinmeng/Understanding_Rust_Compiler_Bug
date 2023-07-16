rust
const fn some_const_fn() {
    if !is_const_eval() {
		return call_at_rt(|| {
			// runtime part
		});
	}
	// const part
}
