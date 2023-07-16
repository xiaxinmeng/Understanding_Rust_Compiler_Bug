toml
changelog-seen = 2

[llvm]
ccache = true
link-shared = true

[build]
build = 'x86_64-unknown-linux-gnu'
host = ['x86_64-unknown-linux-gnu']
target = ['x86_64-unknown-linux-gnu']
cargo = '/usr/bin/cargo'
rustc = '/usr/bin/rustc'
docs = false
compiler-docs = false
vendor = true
extended = true
tools = ['cargo']
configure-args = ['--build=x86_64-unknown-linux-gnu', '--host=x86_64-unknown-linux-gnu', '--target=x86_64-unknown-linux-gnu', '--prefix=/usr', '--bindir=/usr/bin', '--sysconfdir=/etc', '--datadir=/usr/share', '--localstatedir=/var', '--mandir=/usr/share/man', '--infodir=/usr/share/info', '--set', 'rust.deny-warnings=false', '--enable-local-rust', '--libdir=/usr/lib', '--docdir=/usr/share/doc/packages/rust', '--llvm-root=/usr', '--enable-llvm-link-shared', '--disable-codegen-tests', '--enable-optimize', '--enable-ccache', '--disable-docs', '--disable-compiler-docs', '--enable-verbose-tests', '--disable-rpath', '--enable-vendor', '--enable-extended', '--tools=cargo', '--release-channel=stable']

[install]
prefix = '/usr'
sysconfdir = '/etc'
docdir = '/usr/share/doc/packages/rust'
bindir = '/usr/bin'
libdir = '/usr/lib'
mandir = '/usr/share/man'
datadir = '/usr/share'

[rust]
optimize = true
channel = 'stable'
rpath = false
verbose-tests = true
codegen-tests = false
deny-warnings = false

[target.x86_64-unknown-linux-gnu]
llvm-config = '/usr/bin/llvm-config'

[dist]
