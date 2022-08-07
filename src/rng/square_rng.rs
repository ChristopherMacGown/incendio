use super::Rng;

macro_rules! round {
    ($x: ident, $seq: ident) => {{
        let (mut zzz, _): (usize, _) = $x.overflowing_mul($x);
        (zzz, _) = zzz.overflowing_add($seq);

        $x = zzz;
    }};
    ($x: ident, $seq: ident, :rotate) => {{
        round!($x, $seq);

        $x = ($x >> 32) | ($x << 32);
    }};

    ($x: ident, $seq: ident, :shift) => {{
        round!($x, $seq);

        $x >>= 32;
    }};

    ($x: ident, $seq: ident, :into, $t: ident) => {{
        round!($x, $seq);
        $t = $x;

        $x = ($x >> 32) | ($x << 32);
    }};


}

pub struct Squares32((usize, usize));
impl Squares32 {
    pub fn new(counter: usize, key: usize) -> Self {
        Squares32((counter, key))
    }

    pub fn new_from_seed(seed: usize) -> Self {
        Squares32::new(0, seed)
    }

    pub fn next(&mut self) -> u32 {
        let (counter, key) = self.0;

        self.0 .0 += 1;

        let z0 = counter * key; // SquaresRNG - y
        let z1 = z0 + key; // SquaresRNG - z
        let mut z2 = z0;

        round!(z2, z0, :rotate);
        round!(z2, z1, :rotate);
        round!(z2, z0, :rotate);     
        round!(z2, z1, :shift);

        z2 as u32
    }
}

impl Rng for Squares32 {
    type Item = u32;

    fn next(&mut self) -> Self::Item {
        self.next()
    }
}

pub struct Squares64((usize, usize));
impl Squares64 {
    pub fn new(counter: usize, key: usize) -> Self {
        Squares64((counter, key))
    }

    pub fn new_from_seed(seed: usize) -> Self {
        Squares64::new(0, seed)
    }

    pub fn next(&mut self) -> usize {
        let (counter, key) = self.0;

        self.0.0 += 1;

        let z0 = counter * key;
        let z1 = z0 + key;

        let mut z2 = z0;
        let mut z3;

        round!(z2, z0, :rotate); //   round 1
        round!(z2, z1, :rotate); //   round 2
        round!(z2, z0, :rotate); //   round 3
        round!(z2, z1, :into, z3); // round 4
        round!(z2, z0); //            round 5

        z3 ^ (z2 >> 32)
    }
}

impl Rng for Squares64 {
    type Item = usize;

    fn next(&mut self) -> Self::Item {
        self.next()
    }
}

#[cfg(test)]
mod test {
    use crate::rng::well512a;

    use super::{Rng, Squares32, Squares64};
    use std::time::{Duration, Instant};

    #[test]
    fn get_next_squares32() {
        let mut rng = Squares32::new_from_seed(0x5eed);
        let mut rng64 = Squares64::new_from_seed(0x5eed);

        assert_eq!(rng.next(), 0);
        assert_eq!(rng.next(), 0);
        assert_eq!(rng.next(), 0);
        assert_eq!(rng.next(), 4136931726);
        assert_eq!(rng.next(), 3735428845);
        assert_eq!(rng.next(), 310345903);

        assert_eq!(rng64.next() % 100, 1);
        assert_eq!(rng64.next() % 100, 2);
        assert_eq!(rng64.next() % 100, 3);
        assert_eq!(rng64.next() % 100, 24);
        assert_eq!(rng64.next() % 100, 65);
    }

    #[test]
    fn generate_a_million_random_values() {
        fn time_it<R: Rng<Item = usize>>(mut rng: R, count: usize) -> (Duration, usize) {
            let now = Instant::now();
            let average = (1..=count).map(|n| rng.next() / n).sum();

            (now.elapsed(), average)
        }
        fn time_it32<R: Rng<Item = u32>>(mut rng: R, count: usize) -> (Duration, usize) {
            let now = Instant::now();
            let average = (1..=count).map(|n| rng.next() as usize / n).sum();
         
            (now.elapsed(), average)
        }

        let squares32 = Squares32::new_from_seed(0x5eed);
        let squares64 = Squares64::new_from_seed(0x5eed);

        const BIG_NUM: usize = 5_000_000_000;
        const MILLION: usize = 1_000_000;

        println!("Squares64:\t{:.02?}", time_it(squares64  , MILLION));
        println!("Squares32:\t{:.02?}", time_it32(squares32, MILLION));

        assert!(false);
    }
}
