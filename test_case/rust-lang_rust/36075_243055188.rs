 rust
where P: AtRuleParser + DeclarationParser<Declaration = <P as AtRuleParser>::AtRule>
// perhaps even just:
where P: AtRuleParser + DeclarationParser<Declaration = P::AtRule>
