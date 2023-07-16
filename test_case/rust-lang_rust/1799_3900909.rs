
>  vec::islice(v, from, to) // equivalent to v[from:to] in Python
>  vec::islice_from(v, from) // equivalent to v[from:] in Python, or vec::slice(v, from, vec::len(v))
>  vec::islice_to(v, to) // equivalent to v[:to] in Python, or vec::slice(v, 0, to)
> 
>  vec::uslice(v, from, to)
>  vec::uslice_from(v, from, to)
>  vec::uslice_to(v, from, to)
> 