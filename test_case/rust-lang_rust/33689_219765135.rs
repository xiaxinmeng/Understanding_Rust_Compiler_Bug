
    match lhs {
        &TokenTree::Delimited(_, ref tts) => {
            check_matcher(cx, &tts.tts);
        },
        tt @ &TokenTree::Sequence(..) => {
            check_matcher(cx, ref_slice(tt));
        },
        _ => cx.span_err(sp, "invalid macro matcher; matchers must be contained \
                              in balanced delimiters or a repetition indicator")
    };
