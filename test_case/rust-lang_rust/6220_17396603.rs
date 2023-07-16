
 Placeholder := Indicator Position? Flag* Width? Precision? Specifier
 Position := '{' '-'? [0-9]+ '}'
 Width := [1-9] | '[' ('-'? [0-9]+ | '*') ']'
 Precision := '.' ([1-9] | '[' ('-'? [0-9]+ | '*' ) ']')
 Flag := <From supplied set>
 Specifier := <From supplied set>
