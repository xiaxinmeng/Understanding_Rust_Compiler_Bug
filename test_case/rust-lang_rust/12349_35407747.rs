 rust
let (idents, args) = get id and expression list somehow;
let pat = self.ecx.pat(self.fmtsp, ast::PatVec(
                    idents.map(|&x| self.ecx.pat_ident(self.fmtsp, x)),
                    None, ~[]));
let arm = self.ecx.arm(self.fmtsp, ~[pat], body);
let exp = self.ecx.expr_vec_slice(
                    self.fmtsp,
                    args.map(|&x| self.ecx.expr_addr_of(self.fmtsp, x)));
self.ecx.expr_match(self.fmtsp, exp, ~[arm])
