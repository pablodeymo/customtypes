use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct HttpResponseFind<T>
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
