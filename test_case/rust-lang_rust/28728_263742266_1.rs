
An iteration statement whose controlling expression is not a constant
expression, [note 156] that performs no  input/output  operations,
does  not  access  volatile  objects,  and  performs  no synchronization or
atomic operations in its body, controlling expression, or (in the case of
a for statement) its expression-3, may be   assumed   by   the
implementation to terminate. [note 157]

156: An omitted controlling expression is replaced by a nonzero constant,
     which is a constant expression.
157: This  is  intended  to  allow  compiler  transformations  such  as
     removal  of  empty  loops  even  when termination cannot be proven. 
