
gcc/config/rs6000/xcoff.h:

200 /* This is how to store into the string LABEL
201    the symbol_ref name of an internal numbered label where
202    PREFIX is the class of label and NUM is the number within the class.
203    This is suitable for output with `assemble_name'.  */
204 
205 #define ASM_GENERATE_INTERNAL_LABEL(LABEL,PREFIX,NUM)   \
206   sprintf (LABEL, "*%s..%u", rs6000_xcoff_strip_dollar (PREFIX), (unsigned) (NUM))
