bash
    overlay_files=`(cd "$CFG_NON_INSTALLED_OVERLAY" && find . -type f)`
    for f in $overlay_files; do
        # anything
    done
