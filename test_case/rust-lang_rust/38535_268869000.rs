rust
use computed_values::font_family::FontFamily;

pub struct FontFaceRule {
    pub family: FontFamily,
    // ...
}

impl ToCss for FontFaceRule {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
        // ...
        try!(self.family.to_css(dest));
        // ..
    }
}
