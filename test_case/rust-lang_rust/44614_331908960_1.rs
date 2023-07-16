
temp = self.0; // lexpr -> vexpr
scrutinee_temp = &mut temp; // vexpr -> lexpr + coercion
x = &mut (*scrutinee_temp); // match binding
ret_ptr = x; // return value assignment
return
