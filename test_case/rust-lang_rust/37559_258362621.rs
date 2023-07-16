 c
#if __ARM_EABI__
# define COMPILER_RT_ABI __attribute__((pcs("aapcs")))
#else
# define COMPILER_RT_ABI
#endif
