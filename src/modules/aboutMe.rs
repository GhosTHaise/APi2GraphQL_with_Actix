use serde::Serialize;



#[derive(Serialize)]
pub struct Informations {
    name : String,
    last_name : String,
    hobbies : Vec<Hobies>,
    age : u8
}
impl Informations{
    pub fn new(name : String,last_name : String,hobbies : Vec<Hobies>,age : u8) -> Informations{
        Informations { name, last_name, hobbies, age }
    }
}
#[derive(Serialize)]
pub struct Hobies {
    pub id : u8,
    pub name : String
}