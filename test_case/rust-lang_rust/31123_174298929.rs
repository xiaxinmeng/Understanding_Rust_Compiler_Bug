 bash
if [ ! -e ${CFG_SRC_DIR}.git ] || [ -n "$CFG_ENABLE_RUSTBUILD" ]
then
    probe CFG_GIT          git
    msg "git: no git directory. disabling submodules"
    CFG_DISABLE_MANAGE_SUBMODULES=1
