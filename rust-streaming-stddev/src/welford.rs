// use std::fmt:: Debug

#[derive(Debug)]
pub struct Welford {
    m: f64,
    s: f64,
    n: u64,
}

// impl Dis
impl Welford {
    pub fn new() -> Self {
        return Self { m: 0., s: 0., n: 0 };
    }

    pub fn add(self: &mut Self, num: f64) -> () {
        self.n += 1;

        let num_diff = num - self.m;
        println!("num {}", num);
        println!("{:?}", self);

        let m2 = self.m + (num_diff / self.n as f64);
        let s2 = self.s + (num_diff * (num -m2));

        self.m = m2;
        self.s = s2;
    }

    pub fn stddev(self: &Self) -> f64 {
        if self.n == 1 {
            return 0.;
        };
        return (self.s / self.n as f64).sqrt();
    }
}
