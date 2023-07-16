
#[rustc_on_unimplemented(
    on(
        _Self="&str",
        message="found &str but we want a `Column`",
        label="not on a `Column`",
        note="you should be fruxolizing your thingamathings"
    ),
    message="default message"
)]
trait Column {
    type SqlType;
}
