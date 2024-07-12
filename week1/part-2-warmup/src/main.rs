/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

#[warn(unused_imports)]
use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

#[allow(dead_code)]
fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut vec = v;
    for i in &mut vec {
        *i += n;        
    }
    vec
}

#[allow(dead_code)]
fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in v {
        *i += n;
    }
}

#[allow(dead_code)]
fn dedup(v: &mut Vec<i32>) {
    let mut set = HashSet::<i32>::new();
    v.retain(|&x| {
        if set.contains(&x) {
            false
        } else {
            set.insert(x);
            true
        }
    });
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
