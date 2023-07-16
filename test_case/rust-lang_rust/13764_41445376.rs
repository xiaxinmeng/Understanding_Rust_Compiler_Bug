 bash
SCM_REPOSITORY="git://github.com/alexcrichton/rs-splay"
require scm-git
require rust

SUMMARY="A splay tree implementation written in Rust"
HOMEPAGE="https://github.com/alexcrichton/rs-splay"
LICENCES="MIT"
SLOT="0"
PLATFORMS="~amd64 ~x86"
BUGS_TO="jurily@gmail.com"

src_compile() {
    # relative path + crate_id
    rust_lib_compile lib.rs splay
}

src_test() {
    rust_test splay
}

src_install() {
    rust_lib_install splay
}
