bash
while read t; do cargo +nightly test "$t"; done < <(echo 'fix::fix_overlapping
    fix::local_paths
    fix::prepare_for_2018
    fix::specify_rustflags')
