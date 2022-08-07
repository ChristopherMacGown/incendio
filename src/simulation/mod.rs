mod energy;
#[macro_use] mod system;

use energy::Energy;
pub (crate) use system::{System, SystemIterator, WithSimulationExt};


macro_rules! simulation {
    ($name:ident, $decay:expr, $diffuse:expr, $energize:expr) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name;
        impl Simulation for $name {
            type Cell = Energy<u8>;

            fn decay(&self, e: Self::Cell) -> Self::Cell {
                $decay(e)
            }

            fn diffuse(&self, e: Self::Cell) -> Self::Cell {
                $diffuse(e)
            }

            fn energize(&self, e: Self::Cell) -> Self::Cell {
                $energize(e)
            }
        }
    };
}

pub trait Simulation {
    type Cell;

    fn decay(&self, e: Self::Cell) -> Self::Cell;
    fn diffuse(&self, e: Self::Cell) -> Self::Cell;
    fn energize(&self, e: Self::Cell) -> Self::Cell;
}

#[derive(Copy, Clone, Debug)]
pub struct Noop;
impl Simulation for Noop {
    type Cell = Energy<u8>;

    fn decay(&self, e: Self::Cell) -> Self::Cell {
        e
    }

    fn diffuse(&self, e: Self::Cell) -> Self::Cell {
        e
    }

    fn energize(&self, e: Self::Cell) -> Self::Cell {
        e
    }
}

pub struct Fire();

#[cfg(test)]
mod test {
    use super::energy::Energy;
    use super::system::*;
    use super::*;

    #[derive(Copy, Clone, Debug)]
    struct Adder;
    impl Simulation for Adder {
        type Cell = Energy<u8>;

        fn decay(&self, e: Self::Cell) -> Self::Cell {
            e + 1
        }

        fn diffuse(&self, e: Self::Cell) -> Self::Cell {
            e
        }

        fn energize(&self, e: Self::Cell) -> Self::Cell {
            e
        }
    }

    #[test]
    fn test_constructing_a_system() {
        let system = system![1, 2, 3, 4];

        assert_eq!(system.0, [Energy(1), Energy(2), Energy(3), Energy(4)]);
    }

    #[test]
    fn noop_macro_works_the_same_as_the_noop_simulation() {
        simulation!(Noop2, |e| e, |e| e, |e| e);

        let mut system = system![1,2,3,4,5,6].into_iter().take(1);
        let mut system2 = system![1,2,3,4,5,6].into_iter().with_simulation(Noop2).take(1);

        let next = system.next();
        assert_eq!(next, Some(system![1,2,3,4,5,6]));
        assert_eq!(system2.next(), next);
    }

    #[test]
    fn test_iterating_over_a_system() {
        let system = system![1, 2];
        let mut iter = system.into_iter().take(1);

        assert_eq!(iter.next(), Some(system![1, 2]));
        assert_eq!(iter.next(), None);

        let system = system![1, 1, 1, 1];
        let mut iter = system.into_iter().with_simulation(Adder).take(2);

        assert_eq!(iter.next(), Some(system![2, 2, 2, 2]));
        assert_eq!(iter.next(), Some(system![3, 3, 3, 3]));
        assert_eq!(iter.next(), None);
    }
}
