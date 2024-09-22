trying to compute the ackerman function? 

Of the various two-argument versions, the one developed by Péter and Robinson (called "the" Ackermann function by most authors) is defined for nonnegative integers m and n as follows

A(0,n)     = n + 1

A(m+1,0)   = A(m,1)

A(m+1,n+1) = A(m, A(m+1,n))

ackerman grows extremely rapidly

earliest-discovered examples of a total computable function that is not primitive recursive. 

primitive recursive meaning -> can be solved using for loops ie upper bound of every loop is fixed before entering loop

most computable functions are primitive recursive

in fact, for showing that a computable function is primitive recursive, it suffices to show that its time complexity is bounded above by a primitive recursive function of the input size
A(0,n)     = n + 1

A(m,0)   = A(m-1,1)

A(m,n) = A(m-1, A(m,n-1))
1
