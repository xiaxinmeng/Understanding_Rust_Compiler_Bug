
#include <stdio.h>
#include <stdint.h>
#include "positrs.h"

void main(){
	uint32_t a = 0b11000100001110111011110011001000;
	uint32_t b = 0b01011111011001010011000111110001;
	uint32_t c = positdiv32(a,b);
	printf("%d", c);
}
