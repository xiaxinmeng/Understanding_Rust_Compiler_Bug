
fn outlives_forall<T>() where for<'u> T: 'u {}

fn test1<S>() { outlives_forall::<S>(); }

struct Value<'a>(&'a ());
fn test2<'a>() { outlives_forall::<Value<'a>>(); }
