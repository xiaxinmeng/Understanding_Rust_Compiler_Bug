
if let Some(opt) = matches.opt_str("Z") {
    if !is_nightly() {
        return Err("the option `Z` is only accepted on the nightly compiler".into());
    }
    ...
}
