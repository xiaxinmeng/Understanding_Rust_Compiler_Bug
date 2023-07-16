
[00:05:30] warning: diagnostic code E0621 already used
[00:05:30]    --> /checkout/src/librustc/infer/error_reporting/named_anon_conflict.rs:147:13
[00:05:30]     |
[00:05:30] 147 | /             struct_span_err!(self.tcx.sess,
[00:05:30] 148 | |                              span,
[00:05:30] 149 | |                              E0621,
[00:05:30] 150 | |                              "explicit lifetime required in parameter type")
[00:05:30]     | |____________________________________________________________________________^
[00:05:30]     |
[00:05:30] note: previous invocation
[00:05:30]    --> /checkout/src/librustc/infer/error_reporting/named_anon_conflict.rs:134:13
[00:05:30]     |
[00:05:30] 134 | /             struct_span_err!(self.tcx.sess,
[00:05:30] 135 | |                              span,
[00:05:30] 136 | |                              E0621,
[00:05:30] 137 | |                              "explicit lifetime required in the type of `{}`",
[00:05:30] 138 | |                              simple_name)
[00:05:30]     | |_________________________________________^
[00:05:30]     = note: this error originates in a macro outside of the current crate
[00:05:30] 
[00:05:38] error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
[00:05:38]    --> /checkout/src/librustc/infer/error_reporting/named_anon_conflict.rs:110:13
[00:05:38]     |
[00:05:38] 110 |             ty::TyFnDef(_, _, sig) => {
[00:05:38]     |             ^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
[00:05:38] 
