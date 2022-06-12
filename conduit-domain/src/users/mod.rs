use serde::Serialize;

pub mod requests;
pub mod responses;

#[derive(Serialize, Debug)]
pub struct UserDto {
    #[serde(skip_serializing)]
    pub id: i64,
    pub username: String,
    pub email: String,
    pub bio: String,
    pub image: String,
    pub token: String,
}
