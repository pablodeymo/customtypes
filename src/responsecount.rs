use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseCount {
    pub count: i64,
}

impl From<ResponseCount> for HttpResponse {
    fn from(v: ResponseCount) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&v).unwrap())
    }
}
