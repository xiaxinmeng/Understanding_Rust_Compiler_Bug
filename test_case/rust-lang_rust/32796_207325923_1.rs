
#[cfg(A)]      fn a() -> bool { a }
#[cfg(not(A))] fn a() -> bool { true }

#[cfg(B)]      fn b() -> bool { b }
#[cfg(not(B))] fn b() -> bool { true }

if a() && b() && c {
}
