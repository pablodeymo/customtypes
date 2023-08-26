use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseMsg<'msg> {
    pub message: &'msg str,
}

impl<'msg> From<ResponseMsg<'msg>> for HttpResponse {
    fn from(v: ResponseMsg<'msg>) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&v).unwrap())
    }
}
