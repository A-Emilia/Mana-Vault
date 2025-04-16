use crate::model::Card;


pub struct Request {
    pub request_type: RequestType,
}

pub enum RequestType {
    // Is this just for cards? How do I handle putting a card a specific place????
    GetById(usize),
    GetByName(String),
    GetBySet(String),
}

impl Request {
    // I feel like I am just going down a rabbithole of abstraction.
    // Java did a number on me.
    pub fn get_by_id(id: usize) -> Self {
        Self {
        request_type: RequestType::GetById(id)
        }
    }
}
pub struct Response;
pub struct RequestBuilder;
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