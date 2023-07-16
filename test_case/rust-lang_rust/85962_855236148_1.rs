c
if (cpu == 0 && CPU_VAR(PCPU_GET(midr)) == 0 &&
	    CPU_MATCH_ERRATA_CAVIUM_THUNDERX_1_1)
 		printf("WARNING: ThunderX Pass 1.1 detected.\nThis has known "
 		    "hardware bugs that may cause the incorrect operation of "
 		    "atomic operations.\n");
