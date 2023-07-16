 rust
#![feature(test, unicode)]
extern crate test;
use test::Bencher;

extern crate rustc_unicode;

fn main() {
    println!("Hello, world!");
}

fn to_uppercase(this: &str) -> String {
    let mut s = String::with_capacity(this.len());
    let mut left = None;

    // Try to collect slices of upper case characters to push into the
    // result or extend with the upper case version if a lower case
    // character is found.
    for (i, ch) in this.char_indices() {
        if !ch.is_uppercase() {
            if let Some(offset) = left.take() {
                s.push_str(&this[offset..i]);
            }

            s.extend(ch.to_uppercase());
        }
        else if left.is_none() {
            left = Some(i);
        }
    }

    // Append any leftover upper case characters.
    if let Some(offset) = left.take() {
        s.push_str(&this[offset..]);
    }

    s
}

fn to_lowercase(this: &str) -> String {
    let mut s = String::with_capacity(this.len());
    let mut left = None;

    // Try to collect slices of lower case characters to push into the
    // result or extend with the lower case version if an upper case
    // character is found.
    for (i, ch) in this.char_indices() {
        if !ch.is_lowercase() {
            if let Some(offset) = left.take() {
                s.push_str(&this[offset..i]);
            }

            if ch == 'Σ' {
                // Σ maps to σ, except at the end of a word where it maps to ς.
                // This is the only conditional (contextual) but language-independent mapping
                // in `SpecialCasing.txt`,
                // so hard-code it rather than have a generic "condition" mechanism.
                // See https://github.com/rust-lang/rust/issues/26035

                map_uppercase_sigma(this, i, &mut s);
            }
            else {
                s.extend(ch.to_lowercase());
            }
        }
        else if left.is_none() {
            left = Some(i);
        }
    }

    // Append any leftover upper case characters.
    if let Some(offset) = left.take() {
        s.push_str(&this[offset..]);
    }

    return s;

    fn map_uppercase_sigma(from: &str, i: usize, to: &mut String) {
        // See http://www.unicode.org/versions/Unicode7.0.0/ch03.pdf#G33992
        // for the definition of `Final_Sigma`.
        debug_assert!('Σ'.len_utf8() == 2);
        let is_word_final = case_ignoreable_then_cased(from[..i].chars().rev()) &&
                           !case_ignoreable_then_cased(from[i + 2..].chars());

        to.push(if is_word_final {
                'ς'
        } else {
                'σ'
        });
    }

    fn case_ignoreable_then_cased<I: Iterator<Item = char>>(iter: I) -> bool {
        use rustc_unicode::derived_property::{Cased, Case_Ignorable};
        match iter.skip_while(|&c| Case_Ignorable(c)).next() {
            Some(c) => Cased(c),
            None => false,
        }
    }
}

#[test]
fn std_to_lowercase() {
    assert_eq!("".to_lowercase(), "");
    assert_eq!("AÉǅaé ".to_lowercase(), "aéǆaé ");

    // https://github.com/rust-lang/rust/issues/26035
    assert_eq!("ΑΣ".to_lowercase(), "ας");
    assert_eq!("Α'Σ".to_lowercase(), "α'ς");
    assert_eq!("Α''Σ".to_lowercase(), "α''ς");

    assert_eq!("ΑΣ Α".to_lowercase(), "ας α");
    assert_eq!("Α'Σ Α".to_lowercase(), "α'ς α");
    assert_eq!("Α''Σ Α".to_lowercase(), "α''ς α");

    assert_eq!("ΑΣ' Α".to_lowercase(), "ας' α");
    assert_eq!("ΑΣ'' Α".to_lowercase(), "ας'' α");

    assert_eq!("Α'Σ' Α".to_lowercase(), "α'ς' α");
    assert_eq!("Α''Σ'' Α".to_lowercase(), "α''ς'' α");

    assert_eq!("Α Σ".to_lowercase(), "α σ");
    assert_eq!("Α 'Σ".to_lowercase(), "α 'σ");
    assert_eq!("Α ''Σ".to_lowercase(), "α ''σ");

    assert_eq!("Σ".to_lowercase(), "σ");
    assert_eq!("'Σ".to_lowercase(), "'σ");
    assert_eq!("''Σ".to_lowercase(), "''σ");

    assert_eq!("ΑΣΑ".to_lowercase(), "ασα");
    assert_eq!("ΑΣ'Α".to_lowercase(), "ασ'α");
    assert_eq!("ΑΣ''Α".to_lowercase(), "ασ''α");
}

#[test]
fn std_to_uppercase() {
    assert_eq!("".to_uppercase(), "");
    assert_eq!("aéǅßﬁᾀ".to_uppercase(), "AÉǄSSFIἈΙ");
}

#[test]
fn new_to_lowercase() {
    assert_eq!(to_lowercase(""), "");
    assert_eq!(to_lowercase("AÉǅaé "), "aéǆaé ");

    // https://github.com/rust-lang/rust/issues/26035
    assert_eq!(to_lowercase("ΑΣ"), "ας");
    assert_eq!(to_lowercase("Α'Σ"), "α'ς");
    assert_eq!(to_lowercase("Α''Σ"), "α''ς");

    assert_eq!(to_lowercase("ΑΣ Α"), "ας α");
    assert_eq!(to_lowercase("Α'Σ Α"), "α'ς α");
    assert_eq!(to_lowercase("Α''Σ Α"), "α''ς α");

    assert_eq!(to_lowercase("ΑΣ' Α"), "ας' α");
    assert_eq!(to_lowercase("ΑΣ'' Α"), "ας'' α");

    assert_eq!(to_lowercase("Α'Σ' Α"), "α'ς' α");
    assert_eq!(to_lowercase("Α''Σ'' Α"), "α''ς'' α");

    assert_eq!(to_lowercase("Α Σ"), "α σ");
    assert_eq!(to_lowercase("Α 'Σ"), "α 'σ");
    assert_eq!(to_lowercase("Α ''Σ"), "α ''σ");

    assert_eq!(to_lowercase("Σ"), "σ");
    assert_eq!(to_lowercase("'Σ"), "'σ");
    assert_eq!(to_lowercase("''Σ"), "''σ");

    assert_eq!(to_lowercase("ΑΣΑ"), "ασα");
    assert_eq!(to_lowercase("ΑΣ'Α"), "ασ'α");
    assert_eq!(to_lowercase("ΑΣ''Α"), "ασ''α");
}

#[test]
fn new_to_uppercase() {
    assert_eq!(to_uppercase(""), "");
    assert_eq!(to_uppercase("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
}

#[bench]
fn upper_new_really_bad(b: &mut Bencher) {
    b.iter(|| to_uppercase("aAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaA"));
}

#[bench]
fn upper_new_worst(b: &mut Bencher) {
    b.iter(|| to_uppercase("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn upper_new_mixed(b: &mut Bencher) {
    b.iter(|| to_uppercase("AAAaaaaaaaaaaaaAAAAAAAAAaaaaAaaaAAAAaaaaAAaaaaaaaAAAAaaaaa"));
}

#[bench]
fn upper_new_best(b: &mut Bencher) {
    b.iter(|| to_uppercase("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
}

#[bench]
fn upper_new_unicode(b: &mut Bencher) {
    b.iter(|| to_uppercase("AAAßAAAAðAAAAAAæææææAAßAAððððAAAAAAAAAæAÆæAAAAAAAAAAAAAAAA"));
}

#[bench]
fn upper_new_unicode2(b: &mut Bencher) {
    b.iter(|| to_uppercase("aaaßaaaaÐaaaaaaÆÆÆÆÆaaßaaÐÐÐÐaaaaaaaaaÆaæÆaaaaaaaaaaaaaaaa"));
}

#[bench]
fn upper_std_really_bad(b: &mut Bencher) {
    b.iter(|| "aAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaA".to_uppercase());
}

#[bench]
fn upper_std_worst(b: &mut Bencher) {
    b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_uppercase());
}

#[bench]
fn upper_std_mixed(b: &mut Bencher) {
    b.iter(|| "AAAaaaaaaaaaaaaAAAAAAAAAaaaaAaaaAAAAaaaaAAaaaaaaaAAAAaaaaa".to_uppercase());
}

#[bench]
fn upper_std_unicode(b: &mut Bencher) {
    b.iter(|| "AAAßAAAAðAAAAAAæææææAAßAAððððAAAAAAAAAæAÆæAAAAAAAAAAAAAAAA".to_uppercase());
}

#[bench]
fn upper_std_unicode2(b: &mut Bencher) {
    b.iter(|| "aaaßaaaaÐaaaaaaÆÆÆÆÆaaßaaÐÐÐÐaaaaaaaaaÆaæÆaaaaaaaaaaaaaaaa".to_uppercase());
}

#[bench]
fn upper_std_best(b: &mut Bencher) {
    b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_uppercase());
}

#[bench]
fn lower_new_really_bad(b: &mut Bencher) {
    b.iter(|| to_lowercase("AaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAa"));
}

#[bench]
fn lower_new_worst(b: &mut Bencher) {
    b.iter(|| to_lowercase("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"));
}

#[bench]
fn lower_new_mixed(b: &mut Bencher) {
    b.iter(|| to_lowercase("aaaAAAAAAAAAAAAaaaaaaaaaAAAAaAAAaaaaAAAAaaAAAAAAAaaaaAAAAA"));
}

#[bench]
fn lower_new_best(b: &mut Bencher) {
    b.iter(|| to_lowercase("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
}

#[bench]
fn lower_new_unicode(b: &mut Bencher) {
    b.iter(|| to_lowercase("aaaßaaaaÐaaaaaaÆÆÆÆÆaaßaaÐÐÐÐaaaaaaaaaÆaæÆaaaaaaaaaaaaaaaa"));
}

#[bench]
fn lower_new_unicode2(b: &mut Bencher) {
    b.iter(|| to_lowercase("AAAÐªAAAAðAAAAAAæææææAAßAAððððAAAAAAAAAÆAÆÆAAAAAAAAAAAAAAAA"));
}

#[bench]
fn lower_std_really_bad(b: &mut Bencher) {
    b.iter(|| "AaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAaAa".to_lowercase());
}

#[bench]
fn lower_std_worst(b: &mut Bencher) {
    b.iter(|| "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_lowercase());
}

#[bench]
fn lower_std_mixed(b: &mut Bencher) {
    b.iter(|| "aaaAAAAAAAAAAAAaaaaaaaaaAAAAaAAAaaaaAAAAaaAAAAAAAaaaaAAAAA".to_lowercase());
}

#[bench]
fn lower_std_unicode(b: &mut Bencher) {
    b.iter(|| "aaaßaaaaÐaaaaaaÆÆÆÆÆaaßaaÐÐÐÐaaaaaaaaaÆaæÆaaaaaaaaaaaaaaaa".to_lowercase());
}

#[bench]
fn lower_std_unicode2(b: &mut Bencher) {
    b.iter(|| "AAAÐªAAAAðAAAAAAæææææAAßAAððððAAAAAAAAAÆAÆÆAAAAAAAAAAAAAAAA".to_lowercase());
}

#[bench]
fn lower_std_best(b: &mut Bencher) {
    b.iter(|| "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_lowercase());
}
