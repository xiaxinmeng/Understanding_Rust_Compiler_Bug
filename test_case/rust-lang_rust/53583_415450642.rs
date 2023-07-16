
macro diag() {}

#[derive(Diag)] // introduces inert attribute `diag` into scope that shadows `diag`s from outer scopes
#[diag] // refers to the inert attribute
struct S;
