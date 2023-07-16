
<anon>:17:23: 17:66 error: no method named `some_function` found for type `ApiFromC` in the current scope
<anon>:17           (*self.api_from_c).some_function((*self.api_from_c).some_data)
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
<anon>:17:23: 17:66 note: use `((*self.api_from_c).some_function)(...)` if you meant to call the function stored in the `some_function` field
<anon>:17           (*self.api_from_c).some_function((*self.api_from_c).some_data)
