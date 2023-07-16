rust
# Cargo.toml dependency: seed = "0.8"
use seed::prelude::*;

pub fn run() {
    let app = App::start("root", init, update, view);
    move || {
        app.update(Msg);
    };
}


struct Model;
struct Msg;

fn init(_url: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    Node::NoChange
}
