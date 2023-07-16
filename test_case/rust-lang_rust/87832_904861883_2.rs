rust
match None { // line hit 1 time
    Some(_) => 1, // line hit 1 time (testing discriminant)
    None => 0, // line hit 1 time (arm block actually hit)
}
