use domain::aggregate::{
    circle::Circle,
    member::Member,
    value_object::{circle_id::CircleId, member_id::MemberId},
};

use super::member_data::MemberData;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CircleData {
    pub id: i16,
    pub name: String,
    pub owner_id: i16,
    pub owner: MemberData,
    pub capacity: i16,
    pub members: Vec<MemberData>,
}

