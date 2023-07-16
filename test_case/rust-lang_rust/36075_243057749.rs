 diff
-impl<'i, 't, 'a, I, P> Iterator for DeclarationListParser<'i, 't, 'a, I, P>
+impl<'i, 't, 'a, I, P> Iterator for DeclarationListParser<'i, 't, 'a, P>
 where P: DeclarationParser<Declaration = I> + AtRuleParser<AtRule = I> {
     type Item = Result<I, Range<SourcePosition>>;
