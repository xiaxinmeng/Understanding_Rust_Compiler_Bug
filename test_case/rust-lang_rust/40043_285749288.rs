
Type = Path // i.e., a::b
     | Type ('+' Bound)* '+'? // * and ? having same meaning as in a regex
     | "(" Type ")"

Bound = 'a | Path
