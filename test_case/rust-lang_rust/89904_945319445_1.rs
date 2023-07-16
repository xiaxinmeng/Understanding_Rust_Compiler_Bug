rust
// Recursive expansion of StructMeta! macro
// =========================================

#[automatically_derived]
impl::syn::parse::Parse for DisplayArgs {
  fn parse(input: ::syn::parse::ParseStream< '_>) ->  ::syn::Result<Self>{
    let mut _value_0:Option<LitStr>  = None;
    let mut _value_1 = None;
    let mut _value_2 = None;
    let mut _value_3 = None;
    let mut is_next = false;
    let mut unnamed_index = 0;
    let mut named_used = false;
    while!input.is_empty(){
      if is_next {
        input.parse:: < ::syn::Token![,]>()? ;
        if input.is_empty(){
          break;
          
        }
      }is_next = true;
      if let Some((index,span)) =  ::structmeta::helpers::try_parse_name(input, &["dump",],false, &["style",],false, &["bound",],false,false)?{
        named_used = true;
        match index {
          ::structmeta::helpers::NameIndex::Flag(Ok(0usize)) => {
            if _value_3.is_some(){
              return Err(::syn::Error::new(span,"parameter `dump` speficied more than once"));
              
            }_value_3 = Some(span);
            
          }
          ::structmeta::helpers::NameIndex::NameValue(Ok(0usize)) => {
            if _value_1.is_some(){
              return Err(::syn::Error::new(span,"parameter `style` speficied more than once"));
              
            }_value_1 = Some(input.parse:: <LitStr>()?);
            
          }
          ::structmeta::helpers::NameIndex::NameArgs(Ok(0usize)) => {
            if _value_2.is_some(){
              return Err(::syn::Error::new(span,"parameter `bound` speficied more than once"));
              
            }_value_2 = Some({
              let content;
              ::syn::parenthesized!(content in input);
              ::syn::punctuated::Punctuated:: <Quotable<Bound> , ::syn::Token![,]> ::parse_terminated(&content)? .into_iter().collect()
            });
            
          }
          _ => unreachable!()
          
        }
      }else {
        if named_used {
          return Err(input.error("cannot use unnamed parameter after named parameter"));
          
        }match unnamed_index {
          0usize => {
            _value_0 = Some(input.parse:: <LitStr>()?);
            
          }
          _ => {
            return Err(input.error("too many unnamed parameter"));
            
          }
          
        }unnamed_index+=1;
        
      }
    }Ok(Self {
      format:_value_0,style:_value_1,bound:_value_2,dump:_value_3.is_some(),
    })
  }
  
}
