
T = '|' ( id : T ) * '|' [ ':' K ] [ '->' T ]
  | 'proc' '(' ( id : T ) * ')' [ ':' K ] [ '->' T ]
  | ...
K = TraitReference | Lifetime 
