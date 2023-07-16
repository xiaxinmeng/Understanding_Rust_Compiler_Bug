 rust
extern crate mylib;

use mylib::MyTrait;

// signaure is like the functions in src/libsyntax/ext/deriving
fn expand_deriving_mytrait(cx: &mut ExtCtxt,
                           span: Span,
                           mitem: Gc<MetaItem>,
                           item: Gc<Item>,
                           push: |Gc<Item>|) {
    ...
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_deriving(quote_full_path!(MyTrait), expand_deriving_mytrait);
}
