use std::cmp::{min,max};
extern crate scoped_threadpool;
use scoped_threadpool::Pool;

pub fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
    if a.len() != b.len() || a.len() == 0 {
        panic!("Invalid input! Empty vec or different sizes between a and b.");
    }
    let mut pool = Pool::new(8);
    let thread_count = min(20, a.len());
    let elements_per_thread = max(1, a.len() / thread_count);
    let mut partial = vec![];
    partial.resize(thread_count, 0f32);
    pool.scoped(|scope| {
        let mut t = 0;
        for e in &mut partial {
            let start = t * elements_per_thread;
            let end = min(a.len(), start + elements_per_thread);
            scope.execute(move || {
                let mut s = 0f32;
                for i in start..end {
                    s += a[i] * b[i];
                }
                *e = s;
            });
            t += 1;
        }
    });
    let mut sum = 0f32;
    for e in partial.iter() {
        sum += e;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::blas::*;
    #[test]
    fn parallel_dot() {
        let mut a: Vec<f32> = Vec::new();
        let mut b: Vec<f32> = Vec::new();
        let mut s = 0f32;
        for i in 1..101 {
            a.push(i as f32);
            b.push(i as f32);
            s += (i * i) as f32;
        }
        let sum = dot(&a, &b);
        assert_eq!(s, sum);
    }
}
