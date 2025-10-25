use std::fmt;

/*
 * The Request struct used for communicating with the server.
 */
pub struct Request {
    pub method: RequestMethod,
    pub url: String,
    // JSON
    pub content: String,
    
    // A header should be added later with the user making the request.
}

/*
 * All the potential methods for a request.
 */
pub enum RequestMethod {
    Get,
    Post,
    Put,
}

/*
 * Display for RequestMethod because the method in a request is an exhaustive list.
 */
impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestMethod::Get => write!(f, "GET"),
            RequestMethod::Post => write!(f, "POST"),
            RequestMethod::Put => write!(f, "PUT"),
        }
    }
}
