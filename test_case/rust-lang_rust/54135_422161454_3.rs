
EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);

SECTIONS
{
  .vector_table :
  {
    KEEP(*(.vector_table.reset_vector));
    KEEP(*(.vector_table.exceptions));
  }
}
