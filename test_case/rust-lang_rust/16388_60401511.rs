 c
extern __sighandler_t bsd_signal(int, __sighandler_t);          

/* the default is bsd */                                        
static __inline__ __sighandler_t signal(int s, __sighandler_t f)
{                                                               
    return bsd_signal(s,f);                                     
}                                                               
