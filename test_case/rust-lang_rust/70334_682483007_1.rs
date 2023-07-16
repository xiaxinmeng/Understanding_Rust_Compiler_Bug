rust
match data.name() {
    DefPathDataName::Named(name) => write!(f, "{}", name),
    DefPathDataName::Anon { namespace } => write!(f, "{{{}#{}}}", namespace, disambiguator)
}
