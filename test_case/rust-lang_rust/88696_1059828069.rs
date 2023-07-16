
error[E0277]: the trait bound `Result<Option<Score>, build::Error>: From<Result<_, EntryError>>` is not satisfied
   --> src/build.rs:105:33
    |
105 |         Err(EntryError::InvalidScore).into()
    |                                       ^^^^ the trait `From<Result<_, EntryError>>` is not implemented for `Result<Option<Score>, build::Error>`
    |
    = help: the following implementations were found:
              <Result<miniz_oxide::MZStatus, miniz_oxide::MZError> as From<&miniz_oxide::StreamResult>>
              <Result<miniz_oxide::MZStatus, miniz_oxide::MZError> as From<miniz_oxide::StreamResult>>
    = note: required because of the requirements on the impl of `Into<Result<Option<Score>, build::Error>>` for `Result<_, EntryError>`
