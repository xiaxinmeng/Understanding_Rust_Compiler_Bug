rust 
pub fn make_trigger<T, M>(thread: T, regs: ExtiPeriph<M>) -> impl Notifier                                                                     
where                                                                                                                                          
    T: ThrNvic,                                                                                                                                
    M: ExtiMap + ExtiPrPif + ExtiSwierSwiOpt + ExtiSwierSwi,                                                                                   
{                                                                                                                                              
    regs.exti_imr_im.set_bit();                                                                                                                
    let pending = regs.exti_pr_pif;                                                                                                            
    thread.add_fn(move || {                                                                                                                    
        pending.set_bit();                                                                                                                     
        Yielded::<(), ()>(())                                                                                                                  
    });                                                                                                                                        
    thread.enable_int();                                                                                                                       
    SoftIntNotifier(regs.exti_swier_swi)                                                                                                       
}
