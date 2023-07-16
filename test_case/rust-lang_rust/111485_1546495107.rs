rust
use indoc::formatdoc;

fn main() {
    let x = formatdoc! {
        r#"━━━━━━━━
           │ {a} {d}
           │
           │ {longish_name} {c}

           '{e}'
        "#
        , a = "".to_string()
        , longish_name = "".to_string()
        , c = "".to_string()
        , d = "".to_string()
        , e = "".to_string()
    };
}
