pub trait Lattice {
    fn union(&self, other: Self) -> Self; // least upper bound
    fn intersection(&self, other: Self) -> Self; // greatest lower bound
    fn widen(&self, other: Self) -> Self;
    fn narrow(&self, other: Self) -> Self;
}
