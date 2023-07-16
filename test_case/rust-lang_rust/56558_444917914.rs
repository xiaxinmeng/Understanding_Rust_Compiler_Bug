
-Z                       dump-mir=val -- dump MIR state to file
    'val' is used to select which passes and functions to dump. For example:
     - 'all' matches all passes and functions.
     - 'foo' all passes for functions whose name contains 'foo'.
     - 'foo & CleanEndRegions' Only the 'CleanEndRegions' pass for function names containing 'foo'.
     - 'foo | bar' all passes for function names containing 'foo' or 'bar'.
