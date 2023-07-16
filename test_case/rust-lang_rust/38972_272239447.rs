rust
// Before
try!(visitor.visit_key::<__Field>()).map(|impossible| match impossible {});

// After
let None::<__Field> = try!(visitor.visit_key());
