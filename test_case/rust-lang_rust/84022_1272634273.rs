rust
// proc-macro crate
#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
	quote ! { mod generated_mod { struct Generated(TypeFromParent); }
}

// other crate
struct TypeFromParent;

#[derive(MyDeriveMacro)]
struct SomeOtherType;
