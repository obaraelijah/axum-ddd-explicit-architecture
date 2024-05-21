use super::db::Db;

#[derive(Clone, Debug)]
pub struct CircleRepository {
    db: Db,
}

impl CircleRepository {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}