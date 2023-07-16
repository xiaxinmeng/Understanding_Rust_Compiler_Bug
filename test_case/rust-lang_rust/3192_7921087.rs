
1007                 // This might be a struct literal.
1008                 if self.looking_at_record_literal() {
1009 +------ 27 lines: It's a struct literal.---------------------------------------------
1036                 } else if self.look_ahead(1) == token::RBRACE {   // (Code I added starts here)
1037                     // It's an empty struct literal.
1038                     // I'd handle this condition in looking_at_record, but
1039                     // empty records are ambiguous with empty blocks.
1040                     self.bump();
1041                     ex = expr_struct(pth, ~[], none);
1042                     return self.mk_pexpr(lo, hi, ex);   // (ends here)
1043                 }
