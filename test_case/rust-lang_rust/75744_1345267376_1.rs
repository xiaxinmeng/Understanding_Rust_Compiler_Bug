rs
let value = smth.try_fold(Vec::new(), |aggr, curr| {
  // Do something 
  if todo!("Some condition that checks weather aggr is ready for early exit e.g. full") {
    ControlFlow::Break(aggr)
  } else {
    ControlFlow::Continue(aggr)
  }
}).value();
