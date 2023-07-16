rust
// main.rs
use pin_project::pin_project;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

#[pin_project(project = PinProjectTestProj)]
#[derive(Debug)]
pub struct PinProjectTest<Fut>
where
	Fut: Future<Output = ()>,
{
	#[pin]
	fut: Fut,
}

impl<Fut> PinProjectTest<Fut>
where
	Fut: Future<Output = ()>,
{
	pub fn new(fut: Fut) -> Self {
		PinProjectTest {
			fut,
		}
	}
}

impl<Fut> Future for PinProjectTest<Fut>
where
	Fut: Future<Output = ()>,
{
	type Output = Fut::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let PinProjectTestProj { fut } = self.project();

		fut.poll(cx)
	}
}

#[allow(unused_must_use)]
fn main() {
	let fut = async {};
	let test = PinProjectTest::new(fut);

	async {
		test.await;
	};
}
