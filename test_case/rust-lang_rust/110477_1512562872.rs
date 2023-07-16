rust
enum SimplifyCfgPassName {
    Initial,
    PromoteConsts,
    ...
}

impl SimplifyCfg {
  fn new(e: SimplifyCfgPassName) -> Self {
    SimplifyCfg { e }
  }

  fn name(&self) -> &'static str {
    match self.e {
      SimplifyCfgPassName::Initial => "SimplifyCfg-initial",
    }
}
