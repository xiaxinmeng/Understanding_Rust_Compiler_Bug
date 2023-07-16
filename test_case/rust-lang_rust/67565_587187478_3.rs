rust
[DEBUG rustc_mir_build::build::block] ast_block_stmts: local_decls=[                                       
        LocalDecl {                                                                                        
            mutability: Mut,                                                                               
            local_info: Other,                                                                             
            internal: false,                                                                               
            is_block_tail: None,                                                                           
            ty: (),                                                                                        
            user_ty: UserTypeProjections {                                                                 
                contents: [],                                                                              
            },                                                                                             
            source_info: SourceInfo {                                                                      
                span: foo.rs:5:10: 5:10,                                                                   
                scope: scope[0],                                                                           
            },                                                                                             
        },                                                                                                 
        LocalDecl {                                                                                        
            mutability: Not,                                                                               
            local_info: User(                                                                              
                Set(                                                                                       
                    Var(                                                                                   
                        VarBindingForm {                                                                   
                            binding_mode: BindByReference(                                                 
                                Not,                                                                       
                            ),                                                                             
                            opt_ty_info: None,                                                             
                            opt_match_place: Some(                                                         
                                (                                                                          
                                    Some(                                                                  
                                        _3,                                                                
                                    ),                                                                     
                                    foo.rs:6:31: 6:37,                                                     
                                ),                                                                         
                            ),                                                                             
                            pat_span: foo.rs:6:9: 6:28,                                                    
                        },                                                                                 
                    ),                                                                                     
                ),                                                                                         
            ),                                                                                             
            internal: false,                                                                               
            is_block_tail: None,                                                                           
            ty: &mut &mut i32,                                                                             
            user_ty: UserTypeProjections {                                                                 
                contents: [],                                                                              
            },                                                                                             
            source_info: SourceInfo {                                                                      
                span: foo.rs:6:9: 6:28,                                                                    
                scope: scope[0],                                                                           
            },                                                                                             
        },
        LocalDecl {                                                                                        
            mutability: Not,                                                                               
            local_info: User(                                                                              
                Set(                                                                                       
                    Var(                                                                                   
                        VarBindingForm {                                                                   
                            binding_mode: BindByReference(                                                 
                                Not,                                                                       
                            ),                                                                             
                            opt_ty_info: None,                                                             
                            opt_match_place: Some(                                                         
                                (
                                    Some(
                                        _3,
                                    ),
                                    foo.rs:6:31: 6:37,
                                ),
                            ),
                            pat_span: foo.rs:6:9: 6:28,
                        },
                    ),
                ),
            ),
            internal: false,
            is_block_tail: None,
            ty: &&mut i32,
            user_ty: UserTypeProjections {
                contents: [],
            },
            source_info: SourceInfo {
                span: foo.rs:6:22: 6:28,
                scope: scope[0],
            },
        },
        LocalDecl {
            mutability: Mut,
            local_info: Other,
            internal: false,
            is_block_tail: None,
            ty: &mut i32,
            user_ty: UserTypeProjections {
                contents: [],
            },
            source_info: SourceInfo {
                span: foo.rs:6:31: 6:37,
                scope: scope[0],
            },
        },
        LocalDecl {
            mutability: Mut,
            local_info: Other,
            internal: false,
            is_block_tail: None,
            ty: i32,
            user_ty: UserTypeProjections {
                contents: [],
            },
            source_info: SourceInfo {
                span: foo.rs:6:36: 6:37,
                scope: scope[0],
            },
        },
    ]
[DEBUG rustc_mir_build::build::block] ast_block_stmts: var_debug_info=[
        VarDebugInfo {
            name: "_z",
            source_info: SourceInfo {
                span: foo.rs:6:9: 6:28,
                scope: scope[1],
            },
            place: _1,
        },
        VarDebugInfo {
            name: "_a",
            source_info: SourceInfo {
                span: foo.rs:6:22: 6:28,
                scope: scope[1],
            },
            place: _2,
        },
    ]
[DEBUG rustc_mir_build::build::block] ast_block_stmts: var_indices={
        HirId {
            owner: DefIndex(4),
            local_id: 1,
        }: One(
            _1,
        ),
        HirId {
            owner: DefIndex(4),
            local_id: 2,
        }: One(
            _2,
        ),
    }
