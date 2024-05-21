#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(i16);

impl CircleId {
    pub fn gen() -> Self {
        Self(rand::random::<i16>())
    }
}