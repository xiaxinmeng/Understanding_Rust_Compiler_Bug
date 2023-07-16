
fn asinh(x: f64) -> f64 {
	let ax = x.abs();
	let ix = 1.0 / ax;
	((ax + (ax / (f64::hypot(1.0, ix) + ix)))).ln_1p().copysign(x)
}

fn acosh(x: f64) -> f64 {
	if x < 1.0 { f64::NAN } else { (x + ((x - 1.0).sqrt() * (x + 1.0).sqrt())).ln() }
}
