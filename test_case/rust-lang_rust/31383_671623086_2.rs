rust
fn foo<const S: &'static str>(){
       concat_strs!(S, S);
}
