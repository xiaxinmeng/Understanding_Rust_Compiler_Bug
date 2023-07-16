
> expr:
>  | NUMBER 
>  | sum
> 
> sum:
>  | product "+" product
> 
> product:
>  | expr "*" expr
> 