plain
    Checking dirs v4.0.0
    Checking env_logger v0.9.0
    Checking ignore v0.4.17
    Checking thiserror v1.0.30
error: expected `,` following `match` arm
    |
    |
803 |                 Some(context.snippet(self.span).to_owned())
    |                                                            ^ help: missing a comma here to end this `match` arm: `,`

error: expected `,` following `match` arm
    |
    |
806 |                 Some(context.snippet(self.span).to_owned())
    |                                                            ^ help: missing a comma here to end this `match` arm: `,`
error: could not compile `rustfmt-nightly` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustfmt-nightly` due to 2 previous errors
Build completed unsuccessfully in 0:04:31
