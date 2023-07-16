sh
rm -rf build
echo 'profile = "compiler"
changelog-seen = 2

[rust]
codegen-backends = ["cranelift"]' > config.toml
git checkout 9a24e2fee95 # might need to fetch from my remote: https://github.com/jyn514/rust/commit/9a24e2fee959fa9a7c3493489dba0223bfbc8928
x build --stage 0 rustc_codegen_cranelift
git checkout 2d429f3064cb67710fe64dee293329089871d92b
x build --stage 0 rustc_codegen_cranelift
