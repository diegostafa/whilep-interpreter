pub trait Lattice {
    const TOP: Self;
    const BOT: Self;
    const UNIT: Self;

    fn lub(&self, other: &Self) -> Self;
    fn glb(&self, other: &Self) -> Self;
    fn widen(&self, other: &Self) -> Self;
    fn narrow(&self, other: &Self) -> Self;
}
