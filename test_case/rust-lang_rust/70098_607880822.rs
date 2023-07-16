rust
fn main() {
    main1();
}

#[no_mangle]
fn main1() {
	doesnt_work();
	does_work();
}

fn doesnt_work() {
	Executor::new().run(Task::new(do_stuff()));
}

fn does_work() {
	Executor::new().run(Task::new(do_more_stuff()));
}

async fn do_stuff() -> &'static str {
	do_more_stuff().await
}

async fn do_more_stuff() -> &'static str {
	"test"
}
