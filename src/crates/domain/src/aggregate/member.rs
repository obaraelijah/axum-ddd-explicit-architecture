use super::value_object::{grade::Grade, major::Major, member_id::MemberId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub age: i16,
    pub grade: Grade,
    pub major: Major
}

impl Member {
    pub fn new(name: String, age: i16, grade: Grade, major: Major) -> Self {
        Member { 
            id: MemberId::gen(),
            name, 
            age,
            grade,
            major, 
        }
    }
}