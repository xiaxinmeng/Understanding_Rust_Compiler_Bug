
Don't eagerly error on semantically invalid tokens when matching declarative macros, as the input to those doesn't have to be semantically valid.
For attribute/derive proc macros this is not the case, so doing the recovery for them is fine.
