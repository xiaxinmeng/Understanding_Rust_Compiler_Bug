

fn main() {

    concat("nice", "cooel");
    concat("nice".to_string(), "cooel");
    let yop = "nice".to_string();
    concat(&yop, "cooel"); // need to add manually "as &str"
    
}

pub fn concat<S: Into<String>>(path: S, suffix: &str) -> String {
    path.into() + suffix
}
