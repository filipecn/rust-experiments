pub fn fmax(a: f32, b: f32) -> f32 {
    if b.is_nan() || b <= a {
        a
    } else {
        b
    }
}

pub fn fmin(a: f32, b: f32) -> f32 {
    if b.is_nan() || b >= a {
        a
    } else {
        b
    }
}

pub fn trilinear_interp(c: &[[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0f32;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                accum +=
                (i as f32 * u + (1.0 - i as f32) * (1.0 - u)) *
                (j as f32 * v + (1.0 - j as f32) * (1.0 - v)) *
                (k as f32 * w + (1.0 - k as f32) * (1.0 - w)) *
                c[i][j][k];
            }
        }
    }
    accum
}
