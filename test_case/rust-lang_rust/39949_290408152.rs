
println!("Time: {:.2}s", dur.as_secs_f64());
println!("Time: {:.2}s", dur.as_nanos() as f64 * 1e-9);
println!("Time: {}s", dur.display_prec(2));
