
> rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
>  $(filter $(subst *,%,$2),$d))
> 