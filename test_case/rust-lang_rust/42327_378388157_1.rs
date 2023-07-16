rust
error[E0277]: the trait bound `SiteError: std::convert::From<std::path::StripPrefixError>` is not satisfied
   --> src/main.rs:195:14
    |
195 |           url: piece
    |  ______________^
196 | |             .strip_prefix(cur_dir)?
    | |___________________________________^ the trait `std::convert::From<std::path::StripPrefixError>` is not implemented for `SiteError`
    |
    = help: the following implementations were found:
              <SiteError as std::convert::From<std::option::NoneError>>
    = note: required by `std::convert::From::from`

error[E0277]: the trait bound `SiteError: std::convert::From<failure::Error>` is not satisfied
   --> src/main.rs:195:14
    |
195 |           url: piece
    |  ______________^
196 | |             .strip_prefix(cur_dir)?
197 | |             .to_str()
198 | |             .ok_or(err_msg("tostr"))?
    | |_____________________________________^ the trait `std::convert::From<failure::Error>` is not implemented for `SiteError`
    |
    = help: the following implementations were found:
              <SiteError as std::convert::From<std::option::NoneError>>
    = note: required by `std::convert::From::from`
