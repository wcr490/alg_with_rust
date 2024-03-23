use crate::b_rand;

#[allow(dead_code)]
pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p + 1, i);
            v.swap(p, p + 1);
            p += 1;
        }
    }
    p
}

#[allow(dead_code)]
pub fn quick_sort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {}

#[allow(dead_code)]
pub fn threaded_quick_sort<T: 'static + PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    let raw_a: *mut [T] = a as *mut [T];
    let send_a = RawSend(raw_a);
    unsafe {
        let handle = std::thread::spawn(move || {
            threaded_quick_sort(&mut *send_a.0);
        });
        threaded_quick_sort(&mut b[1..]);
        handle.join().ok();
    }
}

#[allow(dead_code)]
pub fn rayon_quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }
    let p = pivot(v);
    let (a, b) = v.split_at_mut(p);
    rayon::join(|| rayon_quick_sort(a), || rayon_quick_sort(&mut b[1..]));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 5, 2, 8, 9, 5, 3, 1];
        // in this example, 3 is the pivot
        // every left element should be smaller than `v[3]`
        // every right element should be bigger than `v[3]`
        let p = pivot(&mut v);
        for i in 1..v.len() {
            assert!((v[i] < v[p]) == (i < p));
        }
    }

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 5, 2, 8, 9, 5, 3, 1];
        quick_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 5, 8, 9])
    }
    #[test]
    fn test_thread_quick_sort() {
        let mut v = vec![4, 5, 2, 8, 9, 5, 3, 1];
        threaded_quick_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 5, 8, 9])
    }
    #[test]
    fn test_rayon_quick_sort() {
        let mut v = vec![4, 5, 2, 8, 9, 5, 3, 1];
        rayon_quick_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 5, 8, 9])
    }
}
