
v: Option<Box<T>> = Some(Box::new(GOOD_VALUE));
v2: Box<T> = move (v as Some).0;
drop(v2);
discr = GetDiscriminant v
