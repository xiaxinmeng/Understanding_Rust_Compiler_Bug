
// one input lifetime parameter, two input lifetimes; elision disallowed
fn test_lifetime_elision_1<'a>(arg1: &'a A, arg2: &'a A) -> &A { 
  arg1
} 

// one input lifetime parameter, two input lifetimes; elision disallowed
fn test_lifetime_elision_2<'a> (arg1: C<'a>, arg2: C<'a>) -> C { 
  arg1
}

// one input lifetime parameter, two input lifetimes; elision allowed
fn test_lifetime_elision_3<'a> (arg1: B<'a, 'a>) -> B { 
  arg1
}

struct A { } 
struct B < 'a, 'b > { 
  a1 : & 'a A , 
  a2 : & 'b A , 
} 
struct C < 'a > { 
  a : & 'a A , 
}
