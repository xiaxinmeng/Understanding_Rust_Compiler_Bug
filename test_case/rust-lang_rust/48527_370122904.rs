
[01:12:28] 2	  --> $DIR/issue-48492-tuple-destructure-missing-parens.rs:48:17
[01:12:28] 3	   |
[01:12:28] 4	LL |     while let b1, b2, b3 = reading_frame.next().expect("there should be a start codon") {
[01:12:28] -	   |               --^------- help: try adding parentheses: `(b1, b2, b3)`
[01:12:28] +	   |               --^
[01:12:28] +	   |               |
[01:12:28] +	   |               help: try adding parentheses: `(b1,)`
