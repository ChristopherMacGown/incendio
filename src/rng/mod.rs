mod well512a;
mod square_rng;
// mod SquareRNG;


pub trait Rng {
    type Item;
    fn next(&mut self) -> Self::Item;
}
