pub trait Mix {
    fn mix(&self, other: &Self) -> Self;
}
