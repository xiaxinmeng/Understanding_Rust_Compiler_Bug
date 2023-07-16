rust
impl Bridge<'_> {                                                                         
    fn enter<R>(self, f: impl FnOnce() -> R) -> R {                                       
        BRIDGE_STATE.with(|s| {                                                           
            let p_enter = s as *const _;                                                  
            dbg!(p_enter);                                                                
        });                                                                               
                                                                                          
        // Hide the default panic output within `proc_macro` expansions.                  
        // NB. the server can't do this because it may use a different libstd.            
        static HIDE_PANICS_DURING_EXPANSION: Once = Once::new();                          
        HIDE_PANICS_DURING_EXPANSION.call_once(|| {                                       
            let prev = panic::take_hook();                                                
            panic::set_hook(Box::new(move |info| {                                        
                let hide = BridgeState::with(|state| match state {                        
                    BridgeState::NotConnected => false,                                   
                    BridgeState::Connected(_) | BridgeState::InUse => true,               
                });                                                                       
                if !hide {                                                                
                    prev(info)                                                            
                }                                                                         
            }));                                                                          
        });                                                                               
                                                                                          
        BRIDGE_STATE.with(|state| state.set(BridgeState::Connected(self), f))             
    }                                                                                     
                                                                                          
    fn with<R>(f: impl FnOnce(&mut Bridge<'_>) -> R) -> R {                               
        BRIDGE_STATE.with(|s| {                                                           
            let p_with = s as *const _;                                                   
            dbg!(p_with);                                                                 
        });                                                                               
                                                                                          
                                                                                          
        BridgeState::with(|state| match state {                                           
            BridgeState::NotConnected => {                                                
                panic!("procedural macro API is used outside of a procedural macro");     
            }                                                                             
            BridgeState::InUse => {                                                       
                panic!("procedural macro API is used while it's already in use");         
            }                                                                             
            BridgeState::Connected(bridge) => f(bridge),                                  
        })                                                                                
    }                                                                                     
}                                                                                         
