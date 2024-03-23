#[allow(dead_code)]
pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
}

#[allow(dead_code)]
pub fn fibonacci_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;
    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }
    res
}

#[allow(dead_code)]
pub fn fibonacci_dy(n: i32) -> (i32, i32) {
    if n == 0 {
        return (1, 0);
    }
    let (a, b) = fibonacci_dy(n - 1);
    //(cur + prev, cur) => (next, cur)
    (a + b, a)
}

fn _fibonacci_tail(n: i32, a: i32, b: i32) -> i32 {
    if n == 0 {
        return a;
    } else if n == 1 {
        return b;
    } else {
        return _fibonacci_tail(n - 1, b, a + b);
    }
}
#[allow(dead_code)]
pub fn fibonacci_tail(n: i32) -> i32 {
    _fibonacci_tail(n, 1, 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fake_dy_fibonacci() {
        for i in 1..15 {
            assert!(fibonacci(i) == fibonacci_iter(i));
        }
    }
    #[test]
    fn test_dy_fibonacci() {
        for i in 1..15 {
            let (res, _) = fibonacci_dy(i);
            assert!(fibonacci(i) == res);
        }
    }
    #[test]
    fn test_tail_fibonacci() {
        for i in 1..15 {
            assert!(fibonacci(i) == fibonacci_tail(i));
        }
    }
}
