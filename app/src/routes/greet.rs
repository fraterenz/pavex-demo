use pavex::request::path::PathParams;
use pavex::response::Response;

#[PathParams]
pub struct GreetParams {
    pub name: String,
}

pub fn get(params: PathParams<GreetParams>) -> Response {
    Response::ok().set_typed_body(format!("Hello {}", params.0.name))
}
