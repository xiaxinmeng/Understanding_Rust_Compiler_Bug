
error: cyclic package dependency: package `compiler_builtins v0.1.19` depends on itself. Cycle:
package `compiler_builtins v0.1.19`
    ... which is depended on by `cfg-if v0.1.10`
    ... which is depended on by `getrandom v0.1.12`
    ... which is depended on by `jobserver v0.1.17`
    ... which is depended on by `cc v1.0.45`
    ... which is depended on by `compiler_builtins v0.1.19`
    ... which is depended on by `alloc v0.0.0 (/home/alex/code/rust2/src/liballoc)`
