rust
let t1 = std:time:SystemTime.now();
// ....long running computation happening...
// System Clock Time changed back 5 hours (because it was wrong to begin with) outside the program
// ....long running computation completed...
let t2 = std:time:SystemTime.now();

let clock_time_between_events = t2.duration_since( t1 ); // returns an error instead of negative duration, so I must do...
let ( clock_time_between_events, neg_duration ) = match ( clock_time_between_events ) {
    Some(duration) => ( duration, false )
    _ => ( t1.duration_since( t2 ).unwrap(), true )
}
