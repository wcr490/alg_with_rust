pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    let mut sorted = true;
    for e in 0..v.len() {
        for i in 0..(v.len() - 1 - e) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}

mod test {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut v = [4, 5, 2, 8, 9, 5, 3, 1];
        bubble_sort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 5, 8, 9]);
    }
}
