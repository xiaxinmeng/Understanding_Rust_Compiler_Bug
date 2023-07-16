
2020-09-19T23:03:26.0942369Z failures:
2020-09-19T23:03:26.0976406Z 
2020-09-19T23:03:26.0977129Z ---- [ui] ui/const-generics/issues/issue-76595.rs stdout ----
2020-09-19T23:03:26.0980716Z diff of stderr:
2020-09-19T23:03:26.0981401Z 
2020-09-19T23:03:26.0982102Z 8	  --> $DIR/issue-76595.rs:15:5
2020-09-19T23:03:26.0982409Z 9	   |
2020-09-19T23:03:26.0982795Z 10	LL | fn test<T, const P: usize>() where Bool<{core::mem::size_of::<T>() > 4}>: True {
2020-09-19T23:03:26.0983417Z -	   |    ---- required by a bound in this
2020-09-19T23:03:26.0984024Z +	   |                                                                           ---- required by this bound in `test`
2020-09-19T23:03:26.0987789Z 12	...
2020-09-19T23:03:26.0988434Z 13	LL |     test::<2>();
2020-09-19T23:03:26.0988748Z 14	   |     ^^^^^^^^^
2020-09-19T23:03:26.0988901Z 
2020-09-19T23:03:26.0989105Z 15	   |
2020-09-19T23:03:26.0989520Z 16	   = note: this may fail depending on what value the parameter takes
2020-09-19T23:03:26.0990389Z -	   = note: required by this bound in `test`
2020-09-19T23:03:26.0990696Z 18	
2020-09-19T23:03:26.0991062Z 19	error: aborting due to 2 previous errors
