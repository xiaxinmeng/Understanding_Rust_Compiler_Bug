mermaid
sequenceDiagram

GCCRS->>myprocmacro: Send a tokenstream
Note right of GCCRS: Transfer using dlopen
myprocmacro-->>GCCRS: Give back a tokenstream
loop CodeExpansion
    myprocmacro->>myprocmacro: Process tokenstream and construct output tokenstream
    libproc_macro->>myprocmacro: Provides tokenstream and other rust types 
end

loop TokenConversion
    GCCRS->>GCCRS: Convert tokens to libproc_macro types back and forth
    libproc_macro_internal->>GCCRS: Provides cpp types for dlopen mechanism
end

libproc_macro->>libproc_macro_internal: Request allocations
libproc_macro_internal-->>libproc_macro: Return allocated type
