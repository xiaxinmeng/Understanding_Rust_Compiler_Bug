rust
	default {
		type Msg: 'static = !;
		fn handle_msg(&mut self, e: Self::Msg) {}
	}
