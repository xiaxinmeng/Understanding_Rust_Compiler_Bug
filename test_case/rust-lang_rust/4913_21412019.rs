 rust
pub type ItemDecorator = @fn(@ExtCtxt,
                             span,
                             @ast::MetaItem,
                             ~[@ast::item])
                          -> ~[@ast::item];
