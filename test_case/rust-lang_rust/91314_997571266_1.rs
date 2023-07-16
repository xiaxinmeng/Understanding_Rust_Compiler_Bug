
error[E0277]: the trait bound `[u8; 1]: Read` is not satisfied
 --> ../scratch/issue-90528-unsizing-suggestion-1.rs:8:16
  |
8 |     wants_read([0u8]);
  |     ---------- ^^^^^ the trait `Read` is not implemented for `[u8; 1]`
  |     |
  |     required by a bound introduced by this call
  |
  = help: the following implementations were found:
            <&[u8] as Read>
note: required by a bound in `wants_read`
 --> ../scratch/issue-90528-unsizing-suggestion-1.rs:5:23
  |
5 | fn wants_read(_: impl Read) {}
  |                       ^^^^ required by this bound in `wants_read`
