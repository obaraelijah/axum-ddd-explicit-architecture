#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleId(i16);

impl CircleId {
    pub fn gen() -> Self {
        Self(rand::random::<i16>())
    }
}

impl std::fmt::Display for CircleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::hash::Hash for CircleId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl std::convert::From<i16> for CircleId {
    fn from(id: i16) -> Self {
        Self(id)
    }
}

impl std::convert::From<CircleId> for i16 {
    fn from(circle_id: CircleId) -> Self {
        circle_id.0 as i16
    }
}

