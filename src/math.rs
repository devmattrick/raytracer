use std::ops::Range;

use rand::prelude::*;

pub fn rand_double(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn clamp(n: f64, range: Range<f64>) -> f64 {
    if n < range.start {
        return range.start;
    }

    if n > range.end {
        return range.end;
    }

    n
}
