pub struct Aligned<T: Into<usize>, const N: usize>(T);
impl<T: Clone + Into<usize>, const N: usize> Aligned<T, N> {
    pub fn from_raw(address: T) -> Option<Self> {
        if address.clone().into() % N == 0 {
            Some(Self(address))
        } else {
            None
        }
    }
}
