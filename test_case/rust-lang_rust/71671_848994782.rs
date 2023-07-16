rust
pub trait Robot {
    type Id;
}

pub type DynRobot = Box<dyn Robot<Id = u32> + Send>;

impl Robot for DynRobot {
    type Id = u32;
}

struct IRobot<R: Robot> {
    id: R::Id,
    robot: R,
}

// stand-in for tokio::spawn
fn this_is_send<T: Send>(value: T) -> T {
    value
}

fn test(source: DynRobot) {
    let _my_task = this_is_send(async move {
        let _my_iter = IRobot {
            id: 32,
            robot: source,
        };
        tokio::task::yield_now().await;
    });
}
