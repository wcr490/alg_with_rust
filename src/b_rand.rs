use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(4999239));
}
pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56664732,
            inc: 348769994,
            modulo: 23257668844,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.modulo;
        self.curr % max
    }
}

#[cfg(test)]
mod test {
    use super::RandGen;

    #[test]
    fn test_rand_generator() {
        let mut r = RandGen::new(44);
        for _ in 0..20 {
            println!("rand: {}", r.next_v(10000000));
        }
        // panic!();
    }
}
