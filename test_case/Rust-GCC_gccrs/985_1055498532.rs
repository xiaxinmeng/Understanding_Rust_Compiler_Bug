rust
154:    fn parse_stmt_mac(&mut self, lo: Span, attrs: AttrVec, path: ast::Path) -> PResult<'a, Stmt> {
155-        let args = self.parse_mac_args()?;
156-        let delim = args.delim();
157-        let hi = self.prev_token.span;
158-
159-        let style =
160-            if delim == token::Brace { MacStmtStyle::Braces } else { MacStmtStyle::NoBraces };
161-
162-        let mac = MacCall { path, args, prior_type_ascription: self.last_type_ascription };
163-
164-        let kind =
165-            if (delim == token::Brace && self.token != token::Dot && self.token != token::Question)
166-                || self.token == token::Semi
167-                || self.token == token::Eof
168-            {
169-                StmtKind::MacCall(P(MacCallStmt { mac, style, attrs, tokens: None }))
170-            } else {
171-                // Since none of the above applied, this is an expression statement macro.
172-                let e = self.mk_expr(lo.to(hi), ExprKind::MacCall(mac), AttrVec::new());
173-                let e = self.maybe_recover_from_bad_qpath(e, true)?;
174-                let e = self.parse_dot_or_call_expr_with(e, lo, attrs.into())?;
175-                let e = self.parse_assoc_expr_with(0, LhsExpr::AlreadyParsed(e))?;
176-                StmtKind::Expr(e)
177-            };
178-        Ok(self.mk_stmt(lo.to(hi), kind))
179-    }
