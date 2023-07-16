
people.sort_by(|a, b| (a.height / a.weigh.pow(2.0)).partial_cmp(b.height / b.weight.pow(2.0))).unwrap()
