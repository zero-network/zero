/// Elliptic curve affine point
pub trait Coordinate {
    fn zero() -> Self;

    fn one() -> Self;

    fn is_zero(&self) -> bool;

    fn is_on_curve(&self) -> bool;
}
