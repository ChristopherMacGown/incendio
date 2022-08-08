use super::Rng;

/// The constant parameters of the WELL512a
const K: usize = 512;
const W: usize = 32;
const R: usize = K / W;
const R_1: usize = R - 1;

const M1: usize = 13;
const M2: usize = 9;

/// The WELL512a is specified against a subset of the transformations in the
/// Transformation Matrix M, defined in the whitepaper. Most saliently to us,
/// are the M₂, M₃, and M₅ transformations. It also uses M₀ but that is zero.
///
/// The M₂ transformation shifts the value x left by t bits (if t is negative)
/// or right by t bits (if t is positive). We only implement M₂ for negative t
/// here, because WELL512a only operates on M₂ for negative values of t.
///
/// The M₃ transformation is defined as the XOR of some value x with the value
/// of itself after it has been shifted left by t bits (if t is negative) or
/// right by t bits (if t is positive).
///
/// The M₅ transformation is defined as the XOR of some value x with the value
/// of the bitwise AND of a specific parameter (b) with the value of x after it
/// has been shifted left by t bits (if t is negative) or right by t bits (if
/// t is positive). We only implement M₅ for negative t here, because the
/// WELL512a generator does not operate on M₅ for positive values of t.

fn mt2_neg(t: isize, x: usize) -> usize {
    x << -t
}

fn mt3_pos(t: usize, x: usize) -> usize {
    // The M₃ transformation for all positive valued t

    x ^ (x >> t)
}

fn mt3_neg(t: isize, x: usize) -> usize {
    // The M₃ transformation for all negative valued t

    x ^ (x << -t)
}

fn mt5_neg(t: isize, b: usize, x: usize) -> usize {
    // The M₅ transformation for all negative valued t
    x ^ ((x << -t) & b)
}

fn map(i: usize, r: usize) -> usize {
    // Always make sure that the index i is within the
    // range of valid indices on the underlying state
    // vector.
    (i + r) & R_1
}

pub(crate) struct Well512a((usize, [usize; R]));
impl Well512a {
    pub fn new(state: [usize; R]) -> Self {
        Well512a((0, state))
    }

    pub fn new_from_seed(seed: usize) -> Self {
        let mut well = Well512a::new([seed; 16]);

        for _ in 0..16 {
            well.next();
        }

        let (_, state) = well.0;

        Well512a((0, state))
    }

    pub fn next(&mut self) -> usize {
        let (index, mut state) = self.0;
        let next = map(index, R_1);

        let z0 = mt3_neg(-16, state[index]) ^ mt3_neg(-15, state[map(index, M1)]);
        let z1 = mt3_pos(11, state[map(index, M2)]);

        state[index] = z0 ^ z1;
        state[next] = mt3_neg(-2, state[next])
            ^ mt3_neg(-18, z0)
            ^ mt2_neg(-28, z1)
            ^ mt5_neg(
                -5,
                0xda442d24, // This is the A₁ value defined in the vector A of specific parameters.
                state[index],
            );

        self.0 = (next, state);
        state[next]
    }
}

impl Rng for Well512a {
    type Item = usize;

    fn next(&mut self) -> Self::Item {
        self.next()
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::{Rng, Well512a};
    use std::time::{Duration, Instant};

    #[test]
    fn get_next() {
        let mut rng = Well512a::new_from_seed(0x5eed); // Test seed.

        assert_eq!((rng.next(), rng.0 .0), (7977243456968075578, 15));
        assert_eq!((rng.next(), rng.0 .0), (8695700250896414010, 14));
        assert_eq!((rng.next(), rng.0 .0), (13289175917187341338, 13));
    }

    #[test]
    fn generate_one_million_random_values() {
        fn time_it<R: Rng<Item = usize>>(mut rng: R, count: usize) -> (Duration, usize) {
            let now = Instant::now();
            let average = (1..=count).map(|_| rng.next() % 3).sum::<usize>();

            (now.elapsed(), average / count)
        }

        let well_512a = Well512a::new_from_seed(0x5eed);
        let (time, average) = time_it(well_512a, 1_000_000);

        assert!(time.as_millis() <= 100);
        assert_eq!(average, 1);
    }
}
