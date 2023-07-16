
wait_while(guard, |x| x.count > 0)
// or
wait_while(guard, |x| !x.started)
