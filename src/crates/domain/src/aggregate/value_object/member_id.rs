#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub struct MemberId(i16);

impl MemberId {
    pub fn gen() -> Self {
        Self(rand::random::<i16>())
    }
}

impl std::hash::Hash for MemberId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::fmt::Display for MemberId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<i16> for MemberId {
    fn from(id: i16) -> Self {
        Self(id)
    }
}

impl std::convert::From<MemberId> for i16 {
    fn from(member_id: MemberId) -> Self {
        member_id.0
    }
}