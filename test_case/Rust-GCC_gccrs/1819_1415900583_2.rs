
gcc/config.gcc:

3094 rs6000-ibm-aix7.1.* | powerpc-ibm-aix7.1.*)
3095         tmake_file="rs6000/t-aix52 t-slibgcc"
3096         if test x$cpu_is_64bit = xyes; then
3097             tm_file="${tm_file} rs6000/biarch64.h"
3098             tmake_file="rs6000/t-aix64 t-slibgcc"
3099         fi
3100         tm_file="${tm_file} rs6000/aix.h rs6000/aix71.h rs6000/xcoff.h rs6000/aix-stdint.h"
3101         extra_options="${extra_options} rs6000/aix64.opt"
3102         use_collect2=yes
3103         thread_file='aix'
3104         use_gcc_stdint=wrap
3105         default_use_cxa_atexit=yes
3106         ;;
3107 rs6000-ibm-aix7.2.* | powerpc-ibm-aix7.2.*)
3108         tmake_file="rs6000/t-aix52 t-slibgcc"
3109         if test x$cpu_is_64bit = xyes; then
3110             tm_file="${tm_file} rs6000/biarch64.h"
3111             tmake_file="rs6000/t-aix64 t-slibgcc"
3112         fi
3113         tm_file="${tm_file} rs6000/aix.h rs6000/aix72.h rs6000/xcoff.h rs6000/aix-stdint.h"
3114         extra_options="${extra_options} rs6000/aix64.opt"
3115         use_collect2=yes
3116         thread_file='aix'
3117         use_gcc_stdint=wrap
3118         default_use_cxa_atexit=yes
3119         ;;
