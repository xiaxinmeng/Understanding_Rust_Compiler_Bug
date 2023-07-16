
impl<T> CheckCfg<T> {
    fn map_data<O: Eq + Hash>(&self, f: impl Fn(&T) -> O) -> CheckCfg<O> {
        CheckCfg {
            names_valid: self
                .names_valid // FxHashSet
                .as_ref()
                .map(|names_valid| names_valid.iter().map(|a| f(a)).collect()),
            values_valid: self
                .values_valid // FxHashMap<T, FxHashSet>
                .iter()
                .map(|(a, b)| (f(a), b.iter().map(|b| f(b)).collect()))
                .collect(),
            well_known_values: self.well_known_values,
        }
    }
}
