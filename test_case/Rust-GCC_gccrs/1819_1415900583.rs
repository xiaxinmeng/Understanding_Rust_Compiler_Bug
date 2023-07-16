
gcc/config/rs6000/rs6000-protos.h:

 48 /* We define this to prevent the name mangler from putting dollar signs into
 49    function names.  */
 50 
 51 #define NO_DOLLAR_IN_LABEL
[...]
329 #ifdef NO_DOLLAR_IN_LABEL
330 const char * rs6000_xcoff_strip_dollar (const char *);
331 #endif
