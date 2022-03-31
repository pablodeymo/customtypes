use actix_web::{body, HttpResponse};
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MsgHttpId {
    pub id: i64,
    #[serde(skip)] // no mostrar en la respuesta
    pub status: u16,
}

impl MsgHttpId {
    pub fn send(id: i64) -> Result<HttpResponse<body::BoxBody>> {
        Ok(HttpResponse::Ok()
            .append_header(("Content-Type", "application/json"))
            .body(serde_json::to_string(&MsgHttpId { id, status: 200 })?))
    }
}
