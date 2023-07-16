
<const-fields> = "u"                        // X
               | "T" {<const>} "E"          // X(a, b, c, ...)
               | {<identifier> <const>} "E" // X { field: value, ... }
