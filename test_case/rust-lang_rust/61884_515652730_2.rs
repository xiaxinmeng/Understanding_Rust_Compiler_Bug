
Ada '83 Language Reference Manual - U.S. Government
http://archive.adaic.com/standards/83lrm/html/lrm-04-05.html   

Integer division and remainder are defined by the relation 

    A = (A/B)*B + (A rem B) 

where (A rem B) has the sign of A and an absolute value less than 
the absolute value of B. Integer division satisfies the identity 

    (-A)/B = -(A/B) = A/(-B) 

The result of the modulus operation is such that (A mod B) has the 
sign of B and an absolute value less than the absolute value of B; 
in addition, for some integer value N, this result must satisfy 
the relation 

    A = B*N + (A mod B) 

...
For positive A and B, A/B is the quotient and A rem B is the 
remainder when A is divided by B. The following relations are 
satisfied by the rem operator: 

    A    rem (-B) =   A rem B
    (-A) rem   B  = -(A rem B) 

For any integer K, the following identity holds: 

    A  mod   B  =  (A + K*B) mod B 

The relations between integer division, remainder, and modulus are
illustrated by the following table: 
