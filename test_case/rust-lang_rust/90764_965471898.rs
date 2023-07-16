
./configure \
  --host=%{_host} \
  --build=%{_build} \
  --prefix=%{_prefix} \
  --bindir=%{_bindir} \
  --sysconfdir=%{_sysconfdir} \
  --datadir=%{_datadir} \
  --localstatedir=%{_localstatedir} \
  --mandir=%{_mandir} \
  --infodir=%{_infodir} \
  --set rust.deny-warnings=false \
  --disable-option-checking \
  --build=%{rust_triple} --host=%{rust_triple} --target=%{rust_triple} \
  --enable-local-rust \
  --local-rust-root=%{rust_root} \
  --libdir=%{common_libdir} \
  --docdir=%{_docdir}/rust \
  --disable-codegen-tests \
  --enable-optimize \
  --disable-docs \
  --disable-compiler-docs \
  --enable-verbose-tests \
  --disable-jemalloc \
  --disable-rpath \
  --enable-vendor \
  --enable-extended \
  --tools="cargo" \
  --release-channel="stable"

cat > .env.sh <<\EOF
export RUSTFLAGS="%{rustflags}"
export DESTDIR=%{buildroot}
export LIBSSH2_SYS_USE_PKG_CONFIG=1
export CARGO_FEATURE_VENDORED=1
# END EXPORTS
EOF

. ./.env.sh

./x.py build -v

./x.py install

./x.py test
