pub fn compress(e: &mut [f64]) -> Vec<f64> {
    compress_expansion(e)
}

fn compress_expansion(e: &mut [f64]) -> Vec<f64> {
    let m = e.len() as i32;
    let mut _q = e[e.len() - 1];
    let mut bottom = m;
    for i in (0..(m - 2 + 1)).rev() {
        let a = _q;
        let b = e[i as usize];
        _q = a + b;
        let bv = _q - a;
        let q = b - bv;
        if q != 0.0 {
            bottom -= 1;
            e[bottom as usize] = _q;
            _q = q;
        }
    }
    let mut top = 0;
    for i in bottom..m {
        let a = e[i as usize];
        let b = _q;
        _q = a + b;
        let bv = _q - a;
        let q = b - bv;
        if q != 0.0 {
            e[top] = q;
            top += 1
        }
    }
    e[top] = _q;
    top += 1;
    e[0..top].to_vec()
}


#[cfg(test)]
mod tes_robust_comp {
    use super::compress;
    #[test]
    fn test_comp() {
        assert_eq!(compress(&mut vec!(0.)), [0.]);
        assert_eq!(compress(&mut vec!(1.)), [1.]);
    }
}
