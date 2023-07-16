 C
/* f32exp_t : Expandable data type definition at language level */

#ifdef AVX_CAPABLE
    #include <immintrin.h>
    typedef f32exp_t __m256;
    #define F32EXP_WIDTH 8
#elif SSE_CAPABLE
    #include <xmmintrin.h>
    typedef f32exp_t __m128;
    #define F32EXP_WIDTH 4
#elif XXX_CAPABLE
    /* Other CPU arch SIMD type with smaller width */
#elif NONE_CAPABLE
    /* Old/unknown hardware fall back */
    typedef f32exp_t float;
    #define F32EXP_WIDTH 1
#endif

/* SIMD operator overloads */
/* Arithmetic addition operator */
/* If AVX_CAPABLE -> _mm256_add_ps -> vaddps */
/* If SSE_CAPABLE -> _mm_add_ps -> addps */
/* If NONE_CAPABLE -> usual add operator */

/* f32exp_t : Expandable data type at language level */

/* Now back to reality at application source code level */
void init(f32exp_t *input)
{
    for (int i=0; i<F32EXP_WIDTH; i++)
    {
        /* f32exp init code */
    }
}

void process(f32exp_t *input1, f32exp_t *input2, f32exp_t *output, int arraySize)
{
    for (int i=0; i<arraySize; i++)
    {
         /* Smart lang/compiler using suitable SIMD arithmetic operator overload */
         output[i] = input1[i] + input2[i];
    }
}
