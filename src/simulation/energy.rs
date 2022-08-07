use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Energy<T: Add<Output = T> + Copy>(pub(crate) T);

impl<T: Add<Output = T> + Copy> Add<T> for Energy<T> {
    type Output = Energy<T>;

    fn add(self, rhs: T) -> Self::Output {
        Energy(self.0 + rhs)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<T> for Energy<T> {
    fn add_assign(&mut self, rhs: T) {
        *self = *self + rhs;
    }
}
