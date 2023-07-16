shell
; x b compiler # clears the directory
; x b miri     # leaves the directory as-is
; x b rustfmt  # leaves the directory as-is

; nano compiler/... # change compiler
; x b miri          # compiler is rebuilt => clears the directory
