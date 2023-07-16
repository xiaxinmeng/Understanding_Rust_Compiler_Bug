rust
use syntax::tokenstream::TokenStream;

// ....

impl base::AttrProcMacro for AttrProcMacro {
    fn expand<'cx>(&self,
                   ecx: &'cx mut ExtCtxt<'_>,
                   span: Span,
                   annotation: TokenStream,
                   annotated: TokenStream)
                   -> TokenStream {
        let server = proc_macro_server::Rustc::new(ecx);
        match self.client.run(&EXEC_STRATEGY, server, annotation, annotated) {
    // ....
}
