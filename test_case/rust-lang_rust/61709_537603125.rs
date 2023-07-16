
#[rustc_on_unimplemented(
   on(all(
       any(from_method="from_error", from_method="from_ok"),
       from_desugaring="QuestionMark"),
      message="the `?` operator can only be used in a \
               function that returns `Result` or `Option` \
               (or another type that implements `{Try}`)",
      label="cannot use the `?` operator in a function that returns `{Self}`",
      enclosing_scope="this should return `Result` or `Option`"), // <--- This is new
   on(all(from_method="into_result", from_desugaring="QuestionMark"),
      message="the `?` operator can only be applied to values \
               that implement `{Try}`",
      label="the `?` operator cannot be applied to type `{Self}`")
)]
#[doc(alias = "?")]
pub trait Try 
