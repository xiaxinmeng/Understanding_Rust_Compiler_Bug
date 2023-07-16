
  let (lastwins, cnts): (Vec<_>, Vec<_>) = { // store outputs in these arrays
    let counter = RelaxedCounter::new();
    restwins.par_iter().map( |r_hi| {
      let out = twins_sieve(*r_hi, kmin, kmax, ks, start_num, end_num, modpg, &primes, &resinvrs);
      print!("\r{} of {} twinpairs done", counter.increment(), pairscnt);
      out
    }).unzip()
  };
