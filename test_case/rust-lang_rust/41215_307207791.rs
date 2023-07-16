rust
a^=c;
c^=d;
b=a;
d=c;
a^=written;
helper::diffuse(a);
helper::diffuse(c);
a^=c;
c^=b;
helper::diffuse(a);
helper::diffuse(c);
return (a,c);
