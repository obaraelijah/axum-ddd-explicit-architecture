#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct MemberId(i16);

impl MemberId {
    pub fn gen() -> Self {
        Self(rand::random::<i16>())
    }
}