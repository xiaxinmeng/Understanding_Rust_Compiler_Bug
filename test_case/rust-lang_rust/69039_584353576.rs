rust
fn p(n: &String, m: &String) -> String {
    println!("PRNT: {} is {}", n, m);
    format!("{} is {}", n, m)
}

fn my_scenario() -> impl Generator<String, Yield = (), Return = ()> {
    |_arg: String| {
        let name1 = yield;
        let name2 = yield;
        p(&name1, &name2);
    }
}
