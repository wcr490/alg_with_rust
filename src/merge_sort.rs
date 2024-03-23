#![allow(dead_code)]
pub fn merge_sort<T: PartialOrd + Copy>(mut v: Vec<T>) -> Vec<T> {
    if v.len() <= 1 {
        return v;
    }
    let mut res: Vec<T> = Vec::with_capacity(v.len());
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_iter = a.into_iter();
    let mut b_iter = b.into_iter();
    let mut a_peek = a_iter.next();
    let mut b_peek = b_iter.next();

    loop {
        match (a_peek, b_peek) {
            (Some(ref a_val), Some(ref b_val)) => {
                if a_val < b_val {
                    res.push(a_peek.take().unwrap());
                    a_peek = a_iter.next();
                } else {
                    res.push(b_peek.take().unwrap());
                    b_peek = b_iter.next();
                }
            }
            (Some(_), None) => {
                res.push(a_peek.take().unwrap());
                res.extend(a_iter);
                return res;
            }
            (None, Some(_)) => {
                res.push(b_peek.take().unwrap());
                res.extend(b_iter);
                return res;
            }
            (None, None) => break,
        }
    }
    res
}

mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 5, 2, 8, 9, 5, 3, 1];
        v = merge_sort(v);
        assert_eq!(v, [1, 2, 3, 4, 5, 5, 8, 9]);
    }
}
