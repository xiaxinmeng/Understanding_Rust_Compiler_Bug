
# curdirs, trailing slashes, and multiple consecutive slashes are collapsed
././wow////nice/ -> wow/nice

# if the result is empty return a single dot
./ -> .

# updirs consume the previous component...
ugh/../cool/ -> cool

# ...but are left prepended if at the start of a relative path
../../../neat -> ../../../neat
