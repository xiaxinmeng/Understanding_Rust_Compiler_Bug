plain
   Compiling clap v4.2.4
   Compiling clap_complete v4.2.2
    Finished dev [unoptimized] target(s) in 12.01s
##[endgroup]
error: unknown option `object-only'
usage: git ls-tree [<options>] <tree-ish> [<path>...]

    -d                    only show trees
    -r                    recurse into subtrees
    -t                    show trees when recursing
    -z                    terminate entries with NUL byte
    -l, --long            include object size
    --name-only           list only filenames
    --name-status         list only filenames
    --full-name           use full path names
    --full-tree           list entire tree; not just current directory (implies --full-name)
    --abbrev[=<n>]        use <n> digits to display object names

thread 'main' panicked at 'command did not execute successfully: cd "/checkout" && "git" "ls-tree" "HEAD" "src/tools/cargo" "--object-only"
expected success, got: exit status: 129', lib.rs:550:27
Build completed unsuccessfully in 0:00:20
