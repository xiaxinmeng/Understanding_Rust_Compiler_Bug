 rust
pub fn crate_disambiguator(&self, cnum: ast::CrateNum) -> token::InternedString {
        if cnum == LOCAL_CRATE {
            self.sess.crate_disambiguator.get().as_str()
        } else {
            self.sess.cstore.crate_name(cnum)
        }
    }
