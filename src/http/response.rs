

pub enum HttpStatus {
    Success = 200,
}

pub struct Response {
    status: HttpStatus,
}

impl Response {
    fn new(status: HttpStatus) -> Response {
        return Response {
            status 
        }
    }
}

