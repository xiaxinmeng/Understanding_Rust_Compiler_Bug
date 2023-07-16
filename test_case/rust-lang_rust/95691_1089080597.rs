rust
#![feature(doc_html_in_header)]
#![doc(html_in_header = r#"
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.15.3/dist/katex.min.css" integrity="sha384-KiWOvVjnN8qwAZbuQyWDIbfCLFhLXNETzBQjA/92pIowpC0d2O3nppDGQVgwd2nB" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.15.3/dist/katex.min.js" integrity="sha384-0fdwu/T/EQMsQlrHCCHoH10pkPLlKA1jL5dFyUOvB3lfeT2540/2g6YgSi2BL14p" crossorigin="anonymous"></script>
<script>
document.addEventListener("DOMContentLoaded", function () {
    let to_do = [];
    for (let e of document.getElementsByTagName('pre')) {
        if (e.classList.contains('language-math')) {
            to_do.push(function () {
                let x = document.createElement('p');
                katex.render(e.innerText, x, {displayMode: true, throwOnError: false});
                e.parentNode.parentNode.replaceChild(x, e.parentNode);
            });
        }
    }
    for (let e of document.getElementsByTagName('code')) {
        let n = e.nextSibling; let p = e.previousSibling;
        if (n && p && /^\$/.test(n.data) && /\$$/.test(p.data)) {
            to_do.push(function () {
                let n = e.nextSibling; let p = e.previousSibling;
                let x = document.createElement('span');
                katex.render(e.innerText, x, {throwOnError: false});
                e.parentNode.replaceChild(x, e);
                n.splitText(1); n.remove();
                p.splitText(p.data.length - 1).remove();
            });
        }
    }
    for (let f of to_do) f();
});
</script>
"#)]

//! This crate is an example of using $`\LaTeX`$ math with rustdoc.
//!
//! This demo uses the unstable `#[doc(html_in_header = ..)]` attribute to
//! inject a KaTeX script into the generated documentation.
//!
//! This way, it works both on docs.rs and with `cargo doc` without extra settings.
//!
//! # Usage
//!
//! Look at the source of `lib.rs` of this crate, and copy the doc attribute
//! containing the `<link>` and `<script>` tags.
//!
//! Then, write ``$`\frac 1 2 + 3`$`` for inline math, which renders as
//! $`\frac 1 2 + 3`$.
//!
//! Or, write
//!
//! 