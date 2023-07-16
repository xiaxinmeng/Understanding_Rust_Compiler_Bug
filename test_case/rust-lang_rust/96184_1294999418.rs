rust
macro_rules! ex1 { ($($a:ident)*, $($b:ident)*) => ($($($a)* $b)*); }
// The outer repetition is defined by $b (only occurrence).
// The inner repetition is defined by $a (only occurrence).
ex1!(a b, x y z) => (a b x a b y a b z);
ex1!(, x y) => (x y);
ex1!(a b, ) => ();

macro_rules! ex2 { ($($a:ident)*, $($b:ident)*) => ($(${ignore(a)} $($a $b)*)*); }
// We use ${ignore(a)} to define the outer repetition according to $a.
// The inner repetition is defined by $b since $a is already used higher in the "stack"/path.
ex2!(a b, x y z) => (a x a y a z b x b y b z);
ex2!(, x y) => ();
ex2!(a b, ) => ();

macro_rules! ex3 { ($($a:ident)*, $($b:ident)*) => ($($a $b)*); }
// This should either be rejected because $a and $b don't share the same input repetition.
// Or we could use the zipping behavior if the lengths are the same. That's apparently the current behavior.

macro_rules! ex4 { ($($a:ident)*) => ($($a $($a)*)*); }
// This should be rejected because $a defines the outer repetition.
// As such, there's no meta-variable remaining to define the inner repetition.

macro_rules! ex5 { ($($a:ident)*, $($b:ident)*) => ($($a $($a $b)*)*); }
// This is a mix of ex2 and ex4.
// $a defines the outer repetition.
// $b defines the inner repetition.
ex5!(a b, x y z) => (a a x a y a z b b x b y b z);
