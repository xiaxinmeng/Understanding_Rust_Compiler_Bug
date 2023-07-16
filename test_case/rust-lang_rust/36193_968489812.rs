
The C99 standard introduces the following datatypes. The documentation can be found here for the AVR stdint library.

uint8_t means it's an 8-bit unsigned type.
uint_fast8_t means it's the fastest unsigned int with at least 8 bits.
uint_least8_t means it's an unsigned int with at least 8 bits.
I understand uint8_t and what is uint_fast8_t( I don't know how it's implemented in register level).

1.Can you explain what is the meaning of "it's an unsigned int with at least 8 bits"?

2.How uint_fast8_t and uint_least8_t help increase efficiency/code space compared to the uint8_t?
