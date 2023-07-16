rust
mod gotham {
	use std::future::Future;

	pub struct State;

	pub trait IntoResponse {}

	pub struct Response;

	impl IntoResponse for Response {}

	mod private {
		use super::*;

		pub trait HandlerMarker {}

		pub trait AsyncHandlerFn<'a> {
			type Res;
		}

		impl<'a, Fut, R, F> AsyncHandlerFn<'a> for F
		where
			F: FnOnce(&'a mut State) -> Fut,
			Fut: Future<Output = R> + Send + 'a
		{
			type Res = R;
		}

		impl<F, R> HandlerMarker for F
		where
			R: IntoResponse + 'static,
			for<'a> F: AsyncHandlerFn<'a, Res = R> + Send
		{
		}
	}

	pub fn to_async_borrowing<F>(handler: F)
	where
		F: private::HandlerMarker
	{
		_ = handler;
	}
}

use gotham::*;

async fn sleep_handler(_: &mut State) -> impl IntoResponse {
	Response
}

fn main() {
	to_async_borrowing(sleep_handler);
}
