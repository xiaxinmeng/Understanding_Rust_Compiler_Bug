
        ResolutionError::MethodNotMemberOfTrait(method, trait_) => {
            use std::ops::{Add, Sub};
            use syntax_pos::Pos;
            use syntax::codemap::BytePos;
            let func_span = Span {
                lo: span.lo.add(BytePos::from_usize(3)),
                hi: span.hi.sub(BytePos::from_usize(5)),
                expn_id: span.expn_id
            };
            let mut err = struct_span_err!(resolver.session,
                                           func_span,
                                           E0407,
                                           "method `{}` is not a member of trait `{}`",
                                           method,
                                           trait_);
            err.span_label(func_span, &format!("not a member of `{}`", trait_));
            err
