
jauhien@zcj rust % checkbashisms configure 
possible bashism in configure line 696 (should be 'b = a'):
    if [ -z "$CC" ] || [[ $CC == *clang ]]
possible bashism in configure line 696 (alternative test command ([[ foo ]] should be [ foo ])):
    if [ -z "$CC" ] || [[ $CC == *clang ]]
