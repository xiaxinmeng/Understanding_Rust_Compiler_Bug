
$ c=fff3a99c9a07beb529205f812dc461e88dc011ee && git log -1 "$c" && git show --raw "$c" | wc -l
commit fff3a99c9a07beb529205f812dc461e88dc011ee
Author: Thomas Schwinge <thomas@codesourcery.com>
Date:   Mon Feb 13 14:24:42 2023 +0100

    Update GCC/Rust files per 'contrib/update-copyright.py --this-year' [#1831]
    
    Fixes: #1831
49
$ contrib/gcc-changelog/git_check_commit.py "$c"
Checking fff3a99c9a07beb529205f812dc461e88dc011ee: FAILED
ERR: cannot find a ChangeLog location in message
