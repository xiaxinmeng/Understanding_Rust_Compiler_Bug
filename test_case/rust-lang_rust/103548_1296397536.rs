sh
; jq '.index | to_entries[] | select(.value.attrs[] | contains("c_variadic")) | .value | select(.name != null) | .name' -C $(rustc --print sysroot)/share/doc/rust/json/core.json 
"VaList"
"VaListImpl"
"VaArgSafe"
