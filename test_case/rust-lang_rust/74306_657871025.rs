
2020-07-13T23:22:10.3748407Z 
2020-07-13T23:22:10.3748573Z failures:
2020-07-13T23:22:10.3827523Z 
2020-07-13T23:22:10.3828420Z ---- [ui] ui/const_prop/ice-assert-fail-div-by-zero.rs stdout ----
2020-07-13T23:22:10.3828644Z diff of stderr:
2020-07-13T23:22:10.3829719Z 
2020-07-13T23:22:10.3830841Z -	warning: this operation will panic at runtime
2020-07-13T23:22:10.3831207Z -	  --> $DIR/ice-assert-fail-div-by-zero.rs:10:5
2020-07-13T23:22:10.3831477Z -	   |
2020-07-13T23:22:10.3831741Z -	LL |     f.0 / 0;
2020-07-13T23:22:10.3832086Z -	   |     ^^^^^^^ attempt to divide _ by zero
2020-07-13T23:22:10.3832348Z -	   |
2020-07-13T23:22:10.3832632Z -	note: the lint level is defined here
2020-07-13T23:22:10.3832948Z -	  --> $DIR/ice-assert-fail-div-by-zero.rs:5:9
2020-07-13T23:22:10.3833209Z -	   |
2020-07-13T23:22:10.3833490Z -	LL | #![warn(unconditional_panic)]
2020-07-13T23:22:10.3833802Z -	   |         ^^^^^^^^^^^^^^^^^^^
2020-07-13T23:22:10.3834058Z -	
2020-07-13T23:22:10.3834333Z -	warning: 1 warning emitted
2020-07-13T23:22:10.3834827Z -	
