console
2019-12-01T22:56:31.7592320Z [TIMING] DocTest { compiler: Compiler { stage: 2, host: "i686-apple-darwin" }, path: "src/doc/rustc", name: "rustc", is_ext_doc: false } -- 4.045
2019-12-01T22:56:32.5534600Z tmp.js:2
2019-12-01T22:56:32.5538260Z searchIndex["alloc"] = {"doc":"The Rust core allocation and collections library","i":[[0,"alloc","alloc","Memory allocation APIs",null,null],[3,"Excess","alloc::alloc","Represents the combination of a starting address and a…",null,null],[12,"0","","",0,null],[12,"1","","",0,null],[3,"Layout","","Layout of a block of memory.",null,null],[3,"LayoutErr","","The parameters given to `Layout::from_size_align` or some…",null,null],[3,"AllocErr","","The `AllocErr` error indicates an allocation failure that…",null,null],[3,"CannotReallocInPlace","","The `CannotReallocInPlace` error is used when…",null,null],[8,"GlobalAlloc","","A memory allocator that can be registered as the standard…",null,null],[10,"alloc","","Allocate memory as described by the given `layout`.",1,[[["self"],["layout"]]]],[10,"dealloc","","Deallocate the block of memory at the given `ptr` pointer…",1,[[["self"],["layout"]]]],[11,"alloc_zeroed","","Behaves like `alloc`, but also ensures that the contents…",1,[[["self"],["lay
2019-12-01T22:56:32.5538820Z 
2019-12-01T22:56:32.5539000Z ReferenceError: addSearchOptions is not defined
2019-12-01T22:56:32.5539090Z     at Object.<anonymous> (tmp.js:2:161112)
2019-12-01T22:56:32.5539200Z     at Module._compile (module.js:577:32)
2019-12-01T22:56:32.5540050Z     at loadContent (/Users/runner/runners/2.160.1/work/1/s/src/tools/rustdoc-js-std/tester.js:166:7)
2019-12-01T22:56:32.5540930Z     at main (/Users/runner/runners/2.160.1/work/1/s/src/tools/rustdoc-js-std/tester.js:264:19)
2019-12-01T22:56:32.5541730Z     at Object.<anonymous> (/Users/runner/runners/2.160.1/work/1/s/src/tools/rustdoc-js-std/tester.js:344:14)
2019-12-01T22:56:32.5541860Z     at Module._compile (module.js:577:32)
2019-12-01T22:56:32.5542000Z     at Object.Module._extensions..js (module.js:586:10)
2019-12-01T22:56:32.5542190Z     at Module.load (module.js:494:32)
2019-12-01T22:56:32.5542290Z     at tryModuleLoad (module.js:453:12)
2019-12-01T22:56:32.5542390Z     at Function.Module._load (module.js:445:3)
2019-12-01T22:56:32.5583130Z 
2019-12-01T22:56:32.5583130Z 
2019-12-01T22:56:32.5584280Z command did not execute successfully: "/usr/local/bin/node" "src/tools/rustdoc-js-std/tester.js" "i686-apple-darwin"
2019-12-01T22:56:32.5584570Z 
2019-12-01T22:56:32.5584610Z 
2019-12-01T22:56:32.5596610Z failed to run: /Users/runner/runners/2.160.1/work/1/s/build/bootstrap/debug/bootstrap test
2019-12-01T22:56:32.5596800Z Build completed unsuccessfully in 1:39:04

