the ackerman function grows insanely fast!!!

i am unable to compute A(4,10) recursively due to stack overflow, need to think of an alternate method;

caching will only get me so far but its worth exploring its limits

omg omg
tried computing A(4,10) using a table cache of 11x11 but it ended up going out of bound due the intermediate values of A(4,10) being more than 10
This is because A(m,n) = A(m-1, A(m,n-1)) the inner Ackerman can return anything, atleast to my current knowledge.
maybe i should try using a hashmap to cache the results let us see how that goes, if we find some pattern the result.
im willingly doing this blind, just knowing the formula for the ackerman function as to not give me wrong ideas or discouragement from trying out stuff!
it is very fun





