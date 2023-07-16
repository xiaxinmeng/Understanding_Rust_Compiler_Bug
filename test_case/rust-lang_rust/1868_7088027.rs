
do task::spawn_unlinked_parented() {
    do task::spawn_unlinked_parented() {
        loop { task::yield(); }
    }
}
fail;
