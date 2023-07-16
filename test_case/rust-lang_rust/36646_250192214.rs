
This code:
uint8_t variable1 = 1;
int8_t variable2 = 2;

Results in this PDB:
(0003E8)  S_BPREL32: [FFFFFFEF], Type:       T_CHAR(0010), variable1
(0003FC)  S_BPREL32: [FFFFFFFB], Type:      T_UCHAR(0020), variable2
