c
> // foo1:
> Demo a = b->foo;
> a.foo = NULL;
> 
> // foo2:
> Demo *a = &b->foo;
> a->foo = NULL;
> 