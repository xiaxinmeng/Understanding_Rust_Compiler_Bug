
BB0:
   temp = <foo>;
   switch temp { 0 => BB1 }
   lifetimeend(temp)
   // if true goes here
   ...
   goto BB2;

BB1:
   // if false
   lifetimeend(temp);
   goto BB2;

BB2:
   ...
