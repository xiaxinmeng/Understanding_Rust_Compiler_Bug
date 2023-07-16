
$ clang++-11 -mmultivalue -Xclang -target-abi -Xclang experimental-mv test.cpp -ObjC++ --compile --target=wasm32-unknown-unknown-wasm --optimize=3 --output test.ll -S -emit-llvm
