
0328e69287b083af4d5d4b49cfc9175e9c82c88e is the first bad commit
commit 0328e69287b083af4d5d4b49cfc9175e9c82c88e
Author: Josh Triplett <josh@joshtriplett.org>
Date:   Sun Oct 25 22:11:20 2020 -0700

    Compile tools and internal libraries with the initial-exec TLS model
    
    This should produce more efficient code, with fewer calls to
    __tls_get_addr. The tradeoff is that libraries using it won't work with
    dlopen, but that shouldn't be a problem for tools or for our own
    internal libraries.
    
    Co-authored-by: Mark Rousskov <mark.simulacrum@gmail.com>

 src/bootstrap/builder.rs | 8 ++++++++
 src/bootstrap/lib.rs     | 4 ++++
 2 files changed, 12 insertions(+)
