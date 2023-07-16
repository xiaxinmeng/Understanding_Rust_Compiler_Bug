
   Compiling‌ clippy_lints v0.0.212 (&#x2F;checkout&#x2F;src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints)‌
error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:105:80‌
    ‌|‌
105‌ ‌| ‌                !self.ignore_fn &amp;&amp; self.eq_expr(l_fun, r_fun) &amp;&amp; self.eq_exprs(l_args, r_args)‌
    ‌| ‌                                                                               ‌^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:105:88‌
    ‌|‌
105‌ ‌| ‌                !self.ignore_fn &amp;&amp; self.eq_expr(l_fun, r_fun) &amp;&amp; self.eq_exprs(l_args, r_args)‌
    ‌| ‌                                                                                       ‌^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:131:90‌
    ‌|‌
131‌ ‌| ‌                !self.ignore_fn &amp;&amp; self.eq_path_segment(l_path, r_path) &amp;&amp; self.eq_exprs(l_args, r_args)‌
    ‌| ‌                                                                                         ‌^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:131:98‌
    ‌|‌
131‌ ‌| ‌                !self.ignore_fn &amp;&amp; self.eq_path_segment(l_path, r_path) &amp;&amp; self.eq_exprs(l_args, r_args)‌
    ‌| ‌                                                                                                 ‌^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:148:85‌
    ‌|‌
148‌ ‌| ‌            (&amp;ExprKind::Tup(ref l_tup), &amp;ExprKind::Tup(ref r_tup)) =&gt; self.eq_exprs(l_tup, r_tup),‌
    ‌| ‌                                                                                    ‌^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:148:92‌
    ‌|‌
148‌ ‌| ‌            (&amp;ExprKind::Tup(ref l_tup), &amp;ExprKind::Tup(ref r_tup)) =&gt; self.eq_exprs(l_tup, r_tup),‌
    ‌| ‌                                                                                           ‌^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:150:81‌
    ‌|‌
150‌ ‌| ‌            (&amp;ExprKind::Array(ref l), &amp;ExprKind::Array(ref r)) =&gt; self.eq_exprs(l, r),‌
    ‌| ‌                                                                                ‌^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:150:84‌
    ‌|‌
150‌ ‌| ‌            (&amp;ExprKind::Array(ref l), &amp;ExprKind::Array(ref r)) =&gt; self.eq_exprs(l, r),‌
    ‌| ‌                                                                                   ‌^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:444:33‌
    ‌|‌
444‌ ‌| ‌                self.hash_exprs(args);‌
    ‌| ‌                                ‌^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:491:33‌
    ‌|‌
491‌ ‌| ‌                self.hash_exprs(args);‌
    ‌| ‌                                ‌^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;utils&#x2F;hir_utils.rs:521:33‌
    ‌|‌
521‌ ‌| ‌                self.hash_exprs(v);‌
    ‌| ‌                                ‌^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
  ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;panic_unimplemented.rs:63:33‌
   ‌|‌
63‌ ‌| ‌                    match_panic(params, expr, cx);‌
   ‌| ‌                                ‌^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
   ‌|‌
   ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌
              found type `‌&amp;rustc::hir::ptr::P&lt;[rustc::hir::Expr]&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;question_mark.rs:142:29‌
    ‌|‌
142‌ ‌| ‌                return Some(ret_expr);‌
    ‌| ‌                            ‌^^^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;rustc::hir::Expr&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;rustc::hir::Expr&gt;‌`‌

error[E0308]‌: mismatched types‌
   ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;question_mark.rs:151:29‌
    ‌|‌
151‌ ‌| ‌                return Some(ret_expr);‌
    ‌| ‌                            ‌^^^^^^^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
    ‌|‌
    ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;rustc::hir::Expr&gt;‌`‌
               found type `‌&amp;rustc::hir::ptr::P&lt;rustc::hir::Expr&gt;‌`‌

error[E0308]‌: mismatched types‌
  ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;redundant_pattern_matching.rs:51:70‌
   ‌|‌
51‌ ‌| ‌                MatchSource::Normal =&gt; find_sugg_for_match(cx, expr, op, arms),‌
   ‌| ‌                                                                     ‌^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
   ‌|‌
   ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;rustc::hir::Expr&gt;‌`‌
              found type `‌&amp;rustc::hir::ptr::P&lt;rustc::hir::Expr&gt;‌`‌

error[E0308]‌: mismatched types‌
  ‌--&gt; ‌src&#x2F;tools&#x2F;clippy&#x2F;clippy_lints&#x2F;src&#x2F;redundant_pattern_matching.rs:52:84‌
   ‌|‌
52‌ ‌| ‌                MatchSource::IfLetDesugar { .. } =&gt; find_sugg_for_if_let(cx, expr, op, arms),‌
   ‌| ‌                                                                                   ‌^^‌ ‌expected struct `syntax::ptr::P`, found struct `rustc::hir::ptr::P`‌
   ‌|‌
   ‌= ‌note‌: expected type `‌&amp;syntax::ptr::P&lt;rustc::hir::Expr&gt;‌`‌
              found type `‌&amp;rustc::hir::ptr::P&lt;rustc::hir::Expr&gt;‌`‌

error‌: aborting due to 16 previous errors‌

For more information about this error, try `rustc --explain E0308`.‌
error:‌ Could not compile `clippy_lints`.‌
warning:‌ build failed, waiting for other jobs to finish...‌
[RUSTC-TIMING] cargo_metadata test:false 23.427
