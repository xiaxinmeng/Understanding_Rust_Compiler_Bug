
there_is_a_bug! { }
// expands to 
there_is_a_bug! { :tt }
// expands to 
there_is_a_bug! { : :tt tt :tt }
// expands to 
there_is_a_bug! { : :tt : :tt tt :tt tt :tt : :tt tt:tt }
// ...
