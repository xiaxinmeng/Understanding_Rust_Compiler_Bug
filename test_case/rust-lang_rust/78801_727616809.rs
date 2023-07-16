
   7   │   unvars_mentioned=Some(
   8   │     {
   9   │         HirId {
  10   │             owner: DefId(0:3 ~ while_let_on_iterator[317d]::issue3670),
  11   │             local_id: 2,
  12   │         }: Upvar {
  13   │             span: $DIR/while_let_on_iterator.rs:14:34: 14:38 (#0),
  14   │         },
  15   │     },
  16   │ ) closure_captures={
  17   │     DefId(0:4 ~ while_let_on_iterator[317d]::issue3670::{closure#0}): {
  18   │         HirId {
  19   │             owner: DefId(0:3 ~ while_let_on_iterator[317d]::issue3670),
  20   │             local_id: 2,
  21   │         }: UpvarId(HirId { owner: DefId(0:3 ~ while_let_on_iterator[317d]::issue3670), local_id: 2 };`ite
       │ r`;DefId(0:4 ~ while_let_on_iterator[317d]::issue3670::{closure#0})),
  22   │     },
  23   │ }
  24   │  min_caps={}
