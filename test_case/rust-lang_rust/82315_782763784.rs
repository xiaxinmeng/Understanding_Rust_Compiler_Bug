rust
fn generates_fonts_css() -> String {
    use std::fmt::Write;
    let fonts = [
        ("Fira Sans", "normal", "400", "Fira Sans", "FiraSans-Regular.woff"),
        ("Fira Sans", "normal", "500", "Fira Sans Medium", "FiraSans-Medium.woff"),
        ("Source Serif Pro", "normal", "400", "Source Serif Pro", "SourceSerifPro-Regular.ttf.woff"),
        ("Source Serif Pro", "italic", "400", "Source Serif Pro Italic", "SourceSerifPro-It.ttf.woff"),
        ("Source Serif Pro", "normal", "700", "Source Serif Pro Bold", "SourceSerifPro-Bold.ttf.woff"),
        ("Source Code Pro", "normal", "400", "", "SourceCodePro-Regular.ttf.woff"),
        ("Source Code Pro", "normal", "600", "", "SourceCodePro-Semibold.ttf.woff"),
    ];
    let mut ret_val = "/*(C) info FONT_LICENSES.txt*/".to_string();
    for (family, style, weight, local, url) in &fonts[..] {
        write!(&mut ret_val, "@font-face{{font-family:'{family}';font-style:{style};font-weight:{weight};src:{maybe_local}url({url})}}",
            family = family,
            style = style,
            weight = weight,
            maybe_local = (if *local != "" { format!("local('{}'),", local) } else { String::new() }),
            url = url
        ).expect("writing to string is infallible");
    }
    assert!(ret_val.len() < 1000); // the exact value is 939, BTW
    ret_val
}
lazy_static!{
    static ref FONTS_CSS: String = generate_fonts_css();
}
