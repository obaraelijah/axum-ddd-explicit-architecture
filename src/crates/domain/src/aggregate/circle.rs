use anyhow::Error;

use super::{
    member::Member,
    value_object::circle_id::CircleId
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Circle {
    pub id: CircleId, // (value Object)
    pub name: String,
    pub capacity: i16,
    pub owner: Member,
    pub members: Vec<Member>,
}

impl Circle {
    pub fn new(name: String, owner: Member, capacity: i16) -> Result<Self, Error> {
        
        Ok(Circle {
            id: CircleId::gen(),
            name,
            owner,
            capacity,
            members: vec![],
        })
    }
}