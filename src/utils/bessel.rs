// Modified bessel functions for first and second king, when x = 0 or 1. Polynomial values copied from https://github.com/SheetJS/bessel 

fn horner(arr: &[f64], x: f64) -> f64 {
    let mut res = 0.0;
    for &val in arr.iter() {
        res = x * res + val;
    }
    res
}

pub fn besselk(x: f64, n: i8) -> f64 {
    fn bessel0(x: f64) -> f64 {
        let b0_a = [0.74e-5, 0.10750e-3, 0.262698e-2, 0.3488590e-1, 0.23069756, 0.42278420, -0.57721566];
        let b0_b = [0.53208e-3, -0.251540e-2, 0.587872e-2, -0.1062446e-1, 0.2189568e-1, -0.7832358e-1, 1.25331414];

        if x <= 2.0 {
            -(x / 2.0).ln() * besseli(x, 0) + horner(&b0_a, x.powi(2) / 4.0)
        } else {
            (-x).exp() / x.sqrt() * horner(&b0_b, 2.0 / x)
        }
    }

    fn bessel1(x: f64) -> f64 {
        let b1_a = [-0.4686e-4, -0.110404e-2, -0.1919402e-1, -0.18156897, -0.67278579, 0.15443144, 1.0];
        let b1_b = [-0.68245e-3, 0.325614e-2, -0.780353e-2, 0.1504268e-1, -0.3655620e-1, 0.23498619, 1.25331414];

        if x <= 2.0 {
            (x / 2.0).ln() * besseli(x, 1) + (1.0 / x) * horner(&b1_a, x.powi(2) / 4.0)
        } else {
            (-x).exp() / x.sqrt() * horner(&b1_b, 2.0 / x)
        }
    }

    //Only 0 and 1 are needed for calculations
    if n == 0 {
        bessel0(x)
    } else if n == 1 {
        bessel1(x)
    } else {
        0.0
    }
}

pub fn besseli(x: f64, n: i8) -> f64 {
    fn bessel0(x: f64) -> f64 {
        let b0_a = [0.45813e-2, 0.360768e-1, 0.2659732, 1.2067492, 3.0899424, 3.5156229, 1.0];
        let b0_b = [0.392377e-2, -0.1647633e-1, 0.2635537e-1, -0.2057706e-1, 0.916281e-2, -0.157565e-2, 0.225319e-2, 0.1328592e-1, 0.39894228];

        if x <= 3.75 {
            horner(&b0_a, x.powi(2) / (3.75*3.75))
        } else {
            (x.abs()).exp() / (x.abs()).sqrt() * horner(&b0_b, 3.75/(x).abs())
        }
    }
    
    fn bessel1(x: f64) -> f64 {
        let b1_a = [0.32411e-3, 0.301532e-2, 0.2658733e-1, 0.15084934, 0.51498869, 0.87890594, 0.5];
        let b1_b = [-0.420059e-2, 0.1787654e-1, -0.2895312e-1, 0.2282967e-1, -0.1031555e-1, 0.163801e-2, -0.362018e-2, -0.3988024e-1, 0.39894228];

        if x <= 3.75 {
            x * horner(&b1_a, x*x/(3.75*3.75))
        } else {
            if x < 1.0 {
                -1.0 * (x.abs()).exp() / (x.abs()).sqrt() * horner(&b1_b, 3.75/(x).abs())
            } else {
                1.0 * (x.abs()).exp() / (x.abs()).sqrt() * horner(&b1_b, 3.75 / (x).abs())
            }
        }
    }

    // Only n = 0 and n = 1 are needed for calculation
    if n == 0 {
        bessel0(x)
    } else if n == 1 {
        bessel1(x)
    } else {
        0.0
    }
}   