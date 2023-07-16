
    1|      1|pub struct Foo(u32);pub struct Foo(u32);                                                                                                                                                                             
    2|       |                                                                                                                                                                                                                     
    3|       |impl Foo {                                                                                                                                                                                                           
    4|      1|    pub fn foo(&self) -> Result<&Foo, ()> {                                                                                                                                                                          
    5|      1|        Ok(self)                                                                                                                                                                                                     
    6|      1|    }                                                                                                                                                                                                                
    7|       |                                                                                                                                                                                                                     
    8|      1|    pub fn bar(&self) -> Result<u32, ()> {                                                                                                                                                                           
    9|      1|        Ok(self.0)                                                                                                                                                                                                   
   10|      1|    }                                                                                                                                                                                                                
   11|       |}                                                                                                                                                                                                                    
   12|       |                                                                                                                                                                                                                     
   13|      1|pub fn bar(f: &Foo) -> Result<u32, ()> {                                                                                                                                                                             
   14|      1|    let x = f                                                                                                                                                                                                        
   15|      1|          .foo()?                                                                                                                                                                                                    
   16|      0|          .bar()?;                                                                                                                                                                                                   
   17|       |                                                                                                                                                                                                                     
   18|      1|    Ok(x + 3)                                                                                                                                                                                                        
   19|      1|}                                                                                                                                                                                                                    
   20|       |                                                                                                                                                                                                                     
   21|       |#[cfg(test)]                                                                                                                                                                                                         
   22|       |mod tests {                                                                                                                                                                                                          
   23|       |    use super::{bar, Foo};                                                                                                                                                                                           
   24|       |                                                                                                                                                                                                                     
   25|       |    #[test]                                                                                                                                                                                                          
   26|      1|    fn test_it() {                                                                                                                                                                                                   
  ------------------                                                                                                                                                                                                               
  | _RNCNvNtCsfrdjj0EOOuS_8kangaroo5tests7test_it0B5_:                                                                                                                                                                             
  |   26|      1|    fn test_it() {                                                                                                                                                                                                
  ------------------                                                                                                                                                                                                               
   27|      1|        let f = Foo(5);                                                                                                                                                                                              
   28|      1|        let result = bar(&f);                                                                                                                                                                                        
   29|      0|        assert_eq!(result, Ok(8))                                                                                                                                                                                    
   30|      1|    }                                                                                                                                                                                                                
  ------------------                                                                                                                                                                                                               
  | _RNvNtCsfrdjj0EOOuS_8kangaroo5testss_7test_it:                                                                                                                                                                                 
  |   26|      1|    fn test_it() {
  |   27|      1|        let f = Foo(5);
  |   28|      1|        let result = bar(&f);
  |   29|      0|        assert_eq!(result, Ok(8))
  |   30|      1|    }
  ------------------
   31|       |}
