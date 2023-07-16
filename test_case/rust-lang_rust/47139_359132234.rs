
commit 3f29e2b495a6b875af835f1fff64256e2d58d2b6
Author: Sebastian Dröge <sebastian@centricular.com>
Date:   Wed Jan 3 12:26:55 2018 +0200

    Fix compilation of TrustedRandomAccess impl for slice::Chunks

    https://github.com/rust-lang/rust/pull/47113 renamed the private size
    field to chunk_size for consistency.
