pub trait Lattice {
    const TOP: Self;
    const BOT: Self;
    const UNIT: Self;

    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn widen(&self, other: &Self) -> Self;
    fn narrow(&self, other: &Self) -> Self;
}
