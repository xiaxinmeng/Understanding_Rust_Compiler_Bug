
#include <avr/interrupt.h>

__attribute__((noinline))
void never_inlined() { asm volatile ("ldi r24, 42" :::"r24"); }

__attribute__((always_inline))
inline void always_inlined() { asm volatile ("ldi r24, 42" :::"r24"); }

ISR(__vector2) { always_inlined(); }
ISR(__vector3) { never_inlined(); }
int main() { return 42; }
