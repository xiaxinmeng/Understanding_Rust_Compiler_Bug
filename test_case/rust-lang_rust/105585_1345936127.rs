rust
PatternError::StaticInPattern(span) => { // EDIT: this code seems redundant?
    self.span_e0158(span, "statics cannot be referenced in patterns")
}
PatternError::AssocConstInPattern(span) => {
    self.span_e0158(span, "associated consts cannot be referenced in patterns")
}
PatternError::ConstParamInPattern(span) => {
    self.span_e0158(span, "const parameters cannot be referenced in patterns")
}
