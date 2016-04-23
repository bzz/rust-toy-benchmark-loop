#![feature(test)]
extern crate test;


#[allow(dead_code)]
pub fn summ_iter() -> f64 {
    let mut sum = 0.0;
    for i in 0..10000000 {
        sum += i as f64;
    }
    return sum
}

#[allow(dead_code)]
pub fn summ_while() -> f64 {
    let mut sum = 0.0;
    let mut i = 0;
    while i < 10000000 {
        sum += i as f64;
        i += 1;
    }
    return sum
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::test::Bencher;

    #[bench]
    fn bench_iter_summ(b: &mut Bencher) {
        b.iter(|| summ_iter());
    }

    #[bench]
    fn bench_while_summ(b: &mut Bencher) {
        b.iter(|| summ_while());
    }
}
