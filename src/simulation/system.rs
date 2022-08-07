use std::iter::Cycle;
pub (crate) use super::energy::Energy;
use super::{Noop, Simulation};

#[macro_export]
macro_rules! system {
    ($h:literal, $($t:literal),+ ) => {
        System([Energy($h), $(Energy($t)),+])
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct System<const N: usize>(pub(crate) [Energy<u8>; N]);

pub trait WithSimulationExt<S: Simulation<Cell = Energy<u8>>> {
    type WithSim;

    fn with_simulation(self, simulation: S) -> Self::WithSim;
}

impl<const N: usize> IntoIterator for System<N> {
    type Item = System<N>;
    type IntoIter = Cycle<SystemIterator<N, Noop>>;

    fn into_iter(self) -> Self::IntoIter {
        SystemIterator {
            system: self,
            simulation: Noop,
        }
        .cycle()
    }
}

#[derive(Clone, Copy)]
pub struct SystemIterator<const N: usize, S: Simulation> {
    system: System<N>,
    simulation: S,
}

impl<const N: usize, S, Z> WithSimulationExt<Z> for Cycle<SystemIterator<N, S>>
where
    S: Simulation<Cell = Energy<u8>> + Clone, // Existing SystemIterator<N, impl Simulation>
    Z: Simulation<Cell = Energy<u8>> + Clone, // Target SystemIterator<N, impl Simulation>
{
    type WithSim = Cycle<SystemIterator<N, Z>>;

    fn with_simulation(mut self, simulation: Z) -> Self::WithSim {
        let system: System<N> = self.next().unwrap();

        SystemIterator { system, simulation }.cycle()
    }
}

impl<const N: usize, S: Simulation<Cell = Energy<u8>>> Iterator for SystemIterator<N, S> {
    type Item = System<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let next: [Energy<u8>; N] = [Energy(0); N];

        self.system.0 = self.system
            .0
            .into_iter()
            .map(|e| self.simulation.decay(e))
            .enumerate()
            .fold(next, |mut acc, (idx, e)| { acc[idx] = e; acc});

        Some(System(self.system.0))
    }
}
