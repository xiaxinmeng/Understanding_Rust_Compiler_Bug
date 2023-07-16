
char_lit : '\x27' char_body '\x27' ;

char_body : non_single_quote
          | '\x5c' [ '\x27' | common_escape | unicode_escape ] ;
