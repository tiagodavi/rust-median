use std::cmp::PartialOrd;
use std::ops::{Add, Div};

fn median<T>(mut values: Vec<T>) -> Option<T>
where
    T: PartialOrd + Add<Output = T> + Div<Output = T> + From<i32>,
{
    if values.is_empty() {
        return None;
    }

    values.sort_by(|a: &T, b: &T| a.partial_cmp(b).unwrap());

    let n_values: usize = values.len();
    let middle_idx: usize = n_values / 2;
    let is_even: bool = n_values % 2 == 0;

    if is_even {
        return Some((values.remove(middle_idx) + values.remove(middle_idx - 1)) / 2.into());
    } else {
        return Some(values.remove(middle_idx));
    }
}

fn main() {
    println!("{:?}", median(vec![9.0, 35.5, 32.6, 58.9, 41.7, 9.3]));
    println!("{:?}", median(vec![9.0, 35.5, 32.6, 58.9, 9.3]));
    println!("{:?}", median(vec![9, 35, 32, 58, 9]));
}
