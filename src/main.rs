use std::collections::HashMap;

fn ackerman_cached(m: usize, n: usize, cache: &mut HashMap<(u32, u32), u32>) -> usize {
    if t[m][n] != 0 {
        return t[m][n];
    }
    let ans;
    if m == 0 {
        ans = n + 1;
    } else if n == 0 {
        ans = ackerman_cached(m - 1, 1, t);
    } else {
        ans = ackerman_cached(m - 1, ackerman_cached(m, n - 1, t), t);
    }
    t[m][n] = ans;
    println!("A[{m}, {n}] => {ans}");
    ans
}

fn main() {
    let mut cache: HashMap<(u32, u32), u32> = HashMap::new();
    println!("{}", ackerman_cached(4, 10, &mut cache));
}

/*
let mut t: [[usize; 11]; 11] = [[0; 11]; 11];
println!("{t:?}");
println!("{}", ackerman(4, 10, &mut t));
*/
fn ackerman_tabulated(m: usize, n: usize, t: &mut [[usize; 11]; 11]) -> usize {
    if t[m][n] != 0 {
        return t[m][n];
    }
    let ans;
    if m == 0 {
        ans = n + 1;
    } else if n == 0 {
        ans = ackerman_tabulated(m - 1, 1, t);
    } else {
        ans = ackerman_tabulated(m - 1, ackerman_tabulated(m, n - 1, t), t);
    }
    t[m][n] = ans;
    println!("A[{m}, {n}] => {ans}");
    ans
}

/*
A(0,n)     = n + 1
A(m,0)   = A(m-1,1)
A(m,n) = A(m-1, A(m,n-1))
*/
