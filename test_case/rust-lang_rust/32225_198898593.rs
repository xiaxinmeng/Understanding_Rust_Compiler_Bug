 rust
if is_global && self.token.is_special_ident_in_modname() {
    return Err(self.fatal(&format!("expected identifier, found special identifier `{}`",
                               self.this_token_to_string())));
}
