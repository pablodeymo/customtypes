use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseFind<T>
where
    T: Serialize,
{
    pub data: std::vec::Vec<T>,
    pub count_pages: i64,
    #[serde(rename(serialize = "recordsTotal"))]
    pub count_total_records: i64,
    #[serde(rename(serialize = "recordsFiltered"))]
    pub count_filtered_records: i64,
}

impl<T: Serialize> From<ResponseFind<T>> for HttpResponse {
    fn from(v: ResponseFind<T>) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&v).unwrap())
    }
}

impl<T: Serialize> From<(Vec<T>, i64, i64)> for ResponseFind<T> {
    fn from(v: (Vec<T>, i64, i64)) -> ResponseFind<T> {
        ResponseFind {
            data: v.0,
            count_pages: v.1,
            count_total_records: v.2,
            count_filtered_records: 0,
        }
    }
}
