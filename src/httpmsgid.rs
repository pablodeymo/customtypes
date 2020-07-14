use actix_web::web;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MsgHttpId {
    pub id: i64,
    #[serde(skip)] // no mostrar en la respuesta
    pub status: u16,
}

impl MsgHttpId {
    pub fn send(id: i64) -> Result<web::HttpResponse> {
        Ok(web::HttpResponse::Ok().json(MsgHttpId { id, status: 200 }))
    }
}
