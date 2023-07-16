
OVERVIEW: 
USAGE: a.out [options]

OPTIONS:

General options:

  -aarch64-neon-syntax                            - Choose style of NEON code to emit from AArch64 backend:
    =generic                                      -   Emit generic NEON assembly
    =apple                                        -   Emit Apple-style NEON assembly
  -bounds-checking-single-trap                    - Use one trap block per function
  -color                                          - use colored syntax highlighting (default=autodetect)
  -cppfname=<function name>                       - Specify the name of the generated function
  -cppfor=<string>                                - Specify the name of the thing to generate
  -cppgen                                         - Choose what kind of output to generate
    =program                                      -   Generate a complete program
    =module                                       -   Generate a module definition
    =contents                                     -   Generate contents of a module
    =function                                     -   Generate a function definition
    =functions                                    -   Generate all function definitions
    =inline                                       -   Generate an inline function
    =variable                                     -   Generate a variable definition
    =type                                         -   Generate a type definition
  -disable-spill-fusing                           - Disable fusing of spill code into instructions
  -enable-implicit-null-checks                    - Fold null checks into faulting memory operations
  -enable-load-pre                                - 
  -enable-objc-arc-opts                           - enable/disable all ARC Optimizations
  -enable-scoped-noalias                          - 
  -enable-tbaa                                    - 
  -exhaustive-register-search                     - Exhaustive Search for registers bypassing the depth and interference cutoffs of last chance recoloring
  -filter-print-funcs=<function names>            - Only print IR for functions whose name match this for all print-[before|after][-all] options
  -gpsize=<uint>                                  - Global Pointer Addressing Size.  The default size is 8.
  -imp-null-check-page-size=<uint>                - The page size of the target in bytes
  -internalize-public-api-file=<filename>         - A file containing list of symbol names to preserve
  -internalize-public-api-list=<list>             - A list of symbol names to preserve
  -join-liveintervals                             - Coalesce copies (default=true)
  -limit-float-precision=<uint>                   - Generate low-precision inline sequences for some float libcalls
  -merror-missing-parenthesis                     - Error for missing parenthesis around predicate registers
  -merror-noncontigious-register                  - Error for register names that aren't contigious
  -mfuture-regs                                   - Enable future registers
  -mips16-constant-islands                        - Enable mips16 constant islands.
  -mips16-hard-float                              - Enable mips16 hard float.
  -mno-compound                                   - Disable looking for compound instructions for Hexagon
  -mno-ldc1-sdc1                                  - Expand double precision loads and stores to their single precision counterparts
  -mno-pairing                                    - Disable looking for duplex instructions for Hexagon
  -mwarn-missing-parenthesis                      - Warn for missing parenthesis around predicate registers
  -mwarn-noncontigious-register                   - Warn for register names that arent contigious
  -mwarn-sign-mismatch                            - Warn for mismatching a signed and unsigned value
  -no-discriminators                              - Disable generation of discriminator information.
  -nvptx-sched4reg                                - NVPTX Specific: schedule for register pressue
  -print-after-all                                - Print IR after each pass
  -print-before-all                               - Print IR before each pass
  -print-machineinstrs=<pass-name>                - Print machine instrs
  -rdf-dump                                       - 
  -rdf-limit=<uint>                               - 
  -regalloc                                       - Register allocator to use
    =default                                      -   pick register allocator based on -O option
    =basic                                        -   basic register allocator
    =fast                                         -   fast register allocator
    =greedy                                       -   greedy register allocator
    =pbqp                                         -   PBQP register allocator
  -rewrite-map-file=<filename>                    - Symbol Rewrite Map
  -rng-seed=<seed>                                - Seed for the random number generator
  -sample-profile-check-record-coverage=<N>       - Emit a warning if less than N% of records in the input profile are matched to the IR.
  -sample-profile-check-sample-coverage=<N>       - Emit a warning if less than N% of samples in the input profile are matched to the IR.
  -sample-profile-global-cold-threshold=<N>       - Top-level functions that account for less than N% of all samples collected in the profile, will be marked as cold for the inliner to consider.
  -sample-profile-global-hot-threshold=<N>        - Top-level functions that account for more than N% of all samples collected in the profile, will be marked as hot for the inliner to consider.
  -sample-profile-inline-hot-threshold=<N>        - Inlined functions that account for more than N% of all samples collected in the parent function, will be inlined again.
  -sample-profile-max-propagate-iterations=<uint> - Maximum number of iterations to go through when propagating sample block/edge weights through the CFG.
  -stackmap-version=<int>                         - Specify the stackmap encoding version (default = 1)
  -stats                                          - Enable statistics output from program (available with Asserts)
  -summary-file=<string>                          - The summary file to use for function importing.
  -time-passes                                    - Time each pass, printing elapsed time for each on exit
  -verify-debug-info                              - 
  -verify-dom-info                                - Verify dominator info (time consuming)
  -verify-loop-info                               - Verify loop info (time consuming)
  -verify-regalloc                                - Verify during register allocation
  -verify-region-info                             - Verify region info (time consuming)
  -verify-scev                                    - Verify ScalarEvolution's backedge taken counts (slow)
  -x86-asm-syntax                                 - Choose style of code to emit from X86 backend:
    =att                                          -   Emit AT&T-style assembly
    =intel                                        -   Emit Intel-style assembly

Generic Options:

  -help                                           - Display available options (-help-hidden for more)
  -help-list                                      - Display list of available options (-help-list-hidden for more)
  -version                                        - Display the version of this program
