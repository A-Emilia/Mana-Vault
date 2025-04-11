
pub struct Request;
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