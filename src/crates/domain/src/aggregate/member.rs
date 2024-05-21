use super::value_object::member_id::MemberId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Member {
    pub id: MemberId,
    pub name: String,
    pub age: i16,
}

impl Member {
    pub fn new(name: String, age: i16) -> Self {
        Member { 
            id: MemberId::gen(),
            name, 
            age, 
        }
    }
}