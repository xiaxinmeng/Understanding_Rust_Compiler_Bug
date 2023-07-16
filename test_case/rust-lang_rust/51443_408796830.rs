
error: the trait `ir::template::TemplateParameters` cannot be made into an object
   --> src/ir/template.rs:134:5
    |
134 | /     fn all_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId>
135 | |     where
136 | |         Self: ItemAncestors,
137 | |     {
...   |
141 | |         }).collect()
142 | |     }
    | |_____^
    |
note: lint level defined here
   --> src/lib.rs:11:9
    |
11  | #![deny(warnings)]
    |         ^^^^^^^^
    = note: #[deny(where_clauses_object_safety)] implied by #[deny(warnings)]
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
    = note: method `all_template_params` references the `Self` type in where clauses

error: the trait `ir::template::TemplateParameters` cannot be made into an object
   --> src/ir/template.rs:147:5
    |
147 | /     fn used_template_params(&self, ctx: &BindgenContext) -> Vec<TypeId>
148 | |     where
149 | |         Self: AsRef<ItemId>,
150 | |     {
...   |
160 | |                     .collect()
161 | |     }
    | |_____^
    |
    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    = note: for more information, see issue #51443 <https://github.com/rust-lang/rust/issues/51443>
    = note: method `used_template_params` references the `Self` type in where clauses

error: aborting due to 2 previous errors

error: Could not compile `bindgen`.
