bash
set -euo pipefail
cd $(mktemp -d)


cargo init -q --name example --lib

cat> src/lib.rs <<'EOF'
    #![allow(unused)]
    struct Foo;
    
    impl Foo {
        async fn f(&self, _: &&()) -> &() {
            &()
        }
    }
EOF

(set -x
    cargo c -q
    { set +x; echo 'enum Bar {}'; } 2>/dev/null | tee -a src/lib.rs
    cargo c -q
)


echo ✅
