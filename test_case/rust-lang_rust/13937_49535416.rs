
((or (looking-at "}") (save-excursion
                        (move-beginning-of-line nil)
                        (looking-at "[ \t]*)")))
 (- baseline rust-indent-offset))
