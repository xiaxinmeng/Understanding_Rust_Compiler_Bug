
2019-11-13T18:34:56.0490492Z extern "C" fn foo(userdata: Box<i32>) {
2019-11-13T18:34:56.0490628Z     /* ... */
2019-11-13T18:34:56.0491056Z 
2019-11-13T18:34:56.0491056Z 
2019-11-13T18:34:56.0491198Z let f: extern "C" fn(*mut i32) = transmute(foo);
2019-11-13T18:34:56.0491353Z callback(f);
2019-11-13T18:34:56.0491599Z 
2019-11-13T18:34:56.0491762Z Here, transmute is being used to convert the types of the fn arguments.
2019-11-13T18:34:56.0491937Z This pattern is incorrect because, because the type of `foo` is a function
2019-11-13T18:34:56.0491937Z This pattern is incorrect because, because the type of `foo` is a function
2019-11-13T18:34:56.0492391Z **item** (`typeof(foo)`), which is zero-sized, and the target type (`fn()`)
2019-11-13T18:34:56.0492934Z is a function pointer, which is not zero-sized.
2019-11-13T18:34:56.0493120Z This pattern should be rewritten. There are a few possible ways to do this:
2019-11-13T18:34:56.0493610Z - change the original fn declaration to match the expected signature,
2019-11-13T18:34:56.0493815Z   and do the cast in the fn body (the preferred option)
2019-11-13T18:34:56.0493815Z   and do the cast in the fn body (the preferred option)
2019-11-13T18:34:56.0494843Z - cast the fn item fo a fn pointer before calling transmute, as shown here:
2019-11-13T18:34:56.0495181Z     