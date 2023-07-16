plain
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
    |
22  | / macro_rules! parse_by_kind {
23  | |     (
24  | |         $self:ident,
25  | |         $expr_id:expr,
...   |
54  | |             _ => return Err($self.expr_error(expr_id, $expected))
55  | |         }
56  | |     }};
57  | | }
    | |_- in this expansion of `parse_by_kind!`
    | |_- in this expansion of `parse_by_kind!`
    |
   ::: compiler/rustc_mir_build/src/build/custom/parse/instruction.rs:142:9
    |
142 | /         parse_by_kind!(self, expr_id, _, "rvalue",
143 | |             @call("mir_discriminant", args) => self.parse_place(args[0]).map(Rvalue::Discriminant),
144 | |             ExprKind::Borrow { borrow_kind, arg } => Ok(
145 | |                 Rvalue::Ref(self.tcx.lifetimes.re_erased, *borrow_kind, self.parse_place(*arg)?)
...   |
150 | |             _ => self.parse_operand(expr_id).map(Rvalue::Use),
    | |             - this pattern is irrefutable; subsequent arms are never executed
    | |_________- in this macro invocation
    |
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: one or more patterns are unreachable
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
    |
22  | / macro_rules! parse_by_kind {
22  | / macro_rules! parse_by_kind {
23  | |     (
24  | |         $self:ident,
25  | |         $expr_id:expr,
...   |
54  | |             _ => return Err($self.expr_error(expr_id, $expected))
55  | |         }
56  | |     }};
57  | | }
    | |_- in this expansion of `parse_by_kind!`
    | |_- in this expansion of `parse_by_kind!`
    |
   ::: compiler/rustc_mir_build/src/build/custom/parse/instruction.rs:155:9
    |
155 | /         parse_by_kind!(self, expr_id, expr, "operand",
156 | |             @call("mir_move", args) => self.parse_place(args[0]).map(Operand::Move),
157 | |             @call("mir_static", args) => self.parse_static(args[0]),
158 | |             @call("mir_static_mut", args) => self.parse_static(args[0]),
...   |
169 | |             _ => self.parse_place(expr_id).map(Operand::Copy),
    | |             - this pattern is irrefutable; subsequent arms are never executed
    | |_________- in this macro invocation

error: one or more patterns are unreachable
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
    |
22  | / macro_rules! parse_by_kind {
23  | |     (
24  | |         $self:ident,
25  | |         $expr_id:expr,
...   |
54  | |             _ => return Err($self.expr_error(expr_id, $expected))
55  | |         }
56  | |     }};
57  | | }
    | |_- in this expansion of `parse_by_kind!`
    | |_- in this expansion of `parse_by_kind!`
    |
   ::: compiler/rustc_mir_build/src/build/custom/parse/instruction.rs:194:17
    |
194 | /                 parse_by_kind!(self, *arg, _, "does not matter",
195 | |                     @call("mir_make_place", args) => return self.parse_place_inner(args[0]),
196 | |                     _ => (*arg, PlaceElem::Deref),
    | |                     - this pattern is irrefutable; subsequent arms are never executed
    | |_________________- in this macro invocation

error: one or more patterns are unreachable
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
   --> compiler/rustc_mir_build/src/build/custom/parse.rs:54:13
    |
22  | / macro_rules! parse_by_kind {
23  | |     (
24  | |         $self:ident,
25  | |         $expr_id:expr,
...   |
54  | |             _ => return Err($self.expr_error(expr_id, $expected))
55  | |         }
56  | |     }};
57  | | }
    | |_- in this expansion of `parse_by_kind!`
    | |_- in this expansion of `parse_by_kind!`
    |
   ::: compiler/rustc_mir_build/src/build/custom/parse/instruction.rs:178:30
    |
178 |           let (parent, proj) = parse_by_kind!(self, expr_id, expr, "place",
    |  ______________________________-
179 | |             @call("mir_field", args) => {
180 | |                 let (parent, ty) = self.parse_place_inner(args[0])?;
181 | |                 let field = Field::from_u32(self.parse_integer_literal(args[1])? as u32);
201 | |             _ => {
201 | |             _ => {
    | |             - this pattern is irrefutable; subsequent arms are never executed
204 | |             },
205 | |         );
    | |_________- in this macro invocation

