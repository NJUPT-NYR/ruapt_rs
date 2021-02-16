mod context;
mod data;

use std::io::Read;

use actix_web::*;

use bendy::serde::to_bytes;
use futures::prelude::*;
use tokio::prelude::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};


#[get("/hello")]
async fn hello_world() -> impl Responder {
    "fuck u"
}

#[get("/announce")]
async fn announce(
    web::Query(q): web::Query<data::AnnounceRequestData>,
    data: web::Data<context::Context>,
) -> impl Responder {
    //TODO: error handler
    let mut con = data.pool.get().await.unwrap();
    let (read_half, write_half) = con.split();
    let mut reader = FramedRead::new(read_half, LengthDelimitedCodec::new());
    let mut writer = FramedWrite::new(write_half, LengthDelimitedCodec::new());
    // TODO: drop passkey(?), check valid(?), route to backend
    let bytes = to_bytes(&q).unwrap();
    writer.send(bytes.into()).await.unwrap();
    if let Ok(Some(msg)) = reader.try_next().await {
        return msg;
    }
    // what should i return?
    panic!("DAMN");
}

pub fn tracker_service() -> Scope {
    web::scope("/tracker")
        // thread safe?
        .data(context::Context::new())
        .service(hello_world)
        .service(announce)
}