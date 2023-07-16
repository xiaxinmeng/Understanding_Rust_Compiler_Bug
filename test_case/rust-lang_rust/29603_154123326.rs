 rust
match name {                                                         
    "appending" => Some(llvm::AppendingLinkage),                     
    "available_externally" => Some(llvm::AvailableExternallyLinkage),
    "common" => Some(llvm::CommonLinkage),                           
    "extern_weak" => Some(llvm::ExternalWeakLinkage),                
    "external" => Some(llvm::ExternalLinkage),                       
    "internal" => Some(llvm::InternalLinkage),                       
    "linkonce" => Some(llvm::LinkOnceAnyLinkage),                    
    "linkonce_odr" => Some(llvm::LinkOnceODRLinkage),                
    "private" => Some(llvm::PrivateLinkage),                         
    "weak" => Some(llvm::WeakAnyLinkage),                            
    "weak_odr" => Some(llvm::WeakODRLinkage),                        
    _ => None,                                                       
}                                                                    
