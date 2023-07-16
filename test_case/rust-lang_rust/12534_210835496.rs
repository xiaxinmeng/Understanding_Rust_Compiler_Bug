
fn merge(l1 : Vec<(String,u32)>, l2 : Vec<(String,u32)>) -> Vec<(String,u32)> {
    let d1 = l1.drain(..);
    let d2 = l2.drain(..);
    let result = Vec::new();
    let mut v1 = d1.next();
    let mut v2 = d2.next();
    loop {
        match (v1,v2) {
            (None,None) => return result,
            (None,Some(x)) => {result.push(x);v2=d2.next()},
            (Some(x),None) => {result.push(x);v1=d1.next()},
            (Some(p1@(s1,t1)),Some(p2@(s2,t2))) => {
                if s1==s2 {
                    result.push((s1,t1+t2));
                    v1=d1.next();
                    v2=d2.next();
                } else if s1<s2 {
                    result.push(p1);
                    v1=d1.next();
                } else {
                    result.push(p2);
                    v2=d2.next();
                }
            },
        }
    }
}
