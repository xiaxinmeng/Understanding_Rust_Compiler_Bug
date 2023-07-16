
#![allow(unused)]

use futures::future::BoxFuture;
use futures::FutureExt;

pub struct GremlinContext;

pub enum Command {
    Print(Option<String>),
    Exec(Box<dyn FnOnce(&GremlinContext) -> BoxFuture<'static, Vec<Command>> + Send>),
}

fn handle(args: Vec<String>) -> Vec<Command> {
    let f = false;

    let task = |_ctx: &GremlinContext| {
        let future = async move {
            let options = f;
            vec![Command::Print(Some("Connected!".into()))]
        };
        future.boxed()
    };

    vec![Command::Exec(Box::new(task))]
}
fn main() {}
