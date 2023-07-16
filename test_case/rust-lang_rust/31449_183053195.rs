
Block A {
    Call(...) on-return B on-unwind C
}

Block B {
    X = CallReturn; // only legal if B has one pred that has a Call terminator
}

Block C {
}
