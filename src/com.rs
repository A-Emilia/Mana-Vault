use crate::model::Card;


pub struct MkOneRequest {
    pub request_type: MkOneRequestType,
}
pub enum MkOneRequestType {
    // Is this just for cards? How do I handle putting a card a specific place????
    Get(MkOneGetType)

}
pub enum MkOneGetType {
    // The response for all Gets should be a list.
    ById(usize),
    ByName(String),
    BySet(String),
}

impl MkOneRequest {
    // I feel like I am just going down a rabbithole of abstraction.
    // Java did a number on me.
    pub fn get_by_id(id: usize) -> Self {
        Self {
        request_type: MkOneRequestType::Get(MkOneGetType::ById(69)),
        }
    }
}

pub struct Request {
    // Could this be made an enum that represents a string? Probably just a hassle.
    method: String,
    url: String,
    // JSON
    content: String,

    // A header should be added later with the user making the request.
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }
}


pub struct Response;
#[derive(Default)]
pub struct RequestBuilder {
    // Optional fields?
}
pub struct ResponseBuilder;


impl RequestBuilder{
    pub fn new() -> Self { todo!() }
    pub fn get(self) -> Self { todo!() }
    pub fn resource(self, arg: &str) -> Self { todo!() }
    pub fn post(self) -> Self { todo!() }
    pub fn build(self) -> Result<Request, ()> { todo!() }
}



mod test{
    #![allow(dead_code, unused_imports, unused_variables)]
    use super::*;
    #[test]
    fn test_request_builder(){
        let request = RequestBuilder::new()
            .get()
            .resource("card1")
            .build()
            .unwrap();
    }
}