
error[E0107]: this enum takes 2 type arguments but only 1 type argument was supplied
  --> $DIR/issue-65159.rs:5:20
   |
LL | async fn copy() -> Result<()>
   |                    ^^^^^^ -- supplied 1 type argument
   |                    |
   |                    expected 2 type arguments
   |
note: enum defined here, with 2 type parameters: `T`, `E`
  --> $SRC_DIR/core/src/result.rs:LL:COL
   |
LL | pub enum Result<T, E> {
   |          ^^^^^^ -  -
help: add missing type argument
   |
LL | async fn copy() -> Result<(), E>
   |                             ^^^
