
pub fn levi_civita_4(i: usize, j: usize, k: usize, l: usize) -> isize {
    if (i != j) && (i != k) && (i != l) && (j != k) && (j != l) && (k != l) {
        let i = i as isize;
        let j = j as isize;
        let k = k as isize;
        let l = l as isize;
    
        ( (i - j) 
        * (i - k) 
        * (i - l) 
        * (j - k) 
        * (j - l) 
        * (k - l)
        ) / 12
    }
    else {
        0
    }
}