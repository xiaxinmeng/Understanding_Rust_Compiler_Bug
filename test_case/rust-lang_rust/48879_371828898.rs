
match self.config.comparison_mode {
    Some(ComparisonMode::NLL) => {
        rustc.args(&["-Znll", "-Zborrowck=mir", "-Ztwo-phase-borrows"]);
    }
    None => { }
}
