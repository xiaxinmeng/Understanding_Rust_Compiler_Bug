 rust
src/raw/sqlconsts.rs:272:48: 272:71 error: cannot refer to other statics by value, use the address-of operator or a cons
tant instead [E0394]                                                                                                    
src/raw/sqlconsts.rs:272 pub static SQL_MAXIMUM_USER_NAME_LENGTH: u16 = *&SQL_MAX_USER_NAME_LEN;                        
                                                                        ^~~~~~~~~~~~~~~~~~~~~~~ 
