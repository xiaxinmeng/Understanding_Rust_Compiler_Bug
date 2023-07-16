
pub trait A: Into<AImpl> {
   //**Association type:**
    type Item<'a>: Scalar<'a> where Self: 'a;
}
pub enum AImpl {
    Int8(P),  // P impl trait A
    Int16(D), // D impl trait A
   //20 Item
}

fn func<'a, T1: A, T2: A, T3: A, T4: A>(a: &'a T1, b: &'a T2, c: &'a T3, d: &'a T4) {
    
}
//
// There is a better way??
//
fn call_func(a:AImpl,b:AImpl,c:AImpl,d:AImpl ) {
    match (a,b,c,d) {
        (  AImpl::Int8(v1),AImpl::Int8(v2),AImpl::Int8(v3),AImpl::Int8(v4),) => {
            func( v1,v2,v3,v4)
        }
       ... //30000 match
    }
}
