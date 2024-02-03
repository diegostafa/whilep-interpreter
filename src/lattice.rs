pub trait Lattice {
    fn union(&self, other: Self) -> Self;
    fn intersection(&self, other: Self) -> Self;
    fn widen(&self, other: Self) -> Self;
    fn narrow(&self, other: Self) -> Self;
}
