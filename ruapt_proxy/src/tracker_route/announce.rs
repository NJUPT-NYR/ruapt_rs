use crate::config::CONFIG;
use crate::ProxyResult;
use crate::error::ProxyError;

use super::context::Context;
use super::data::{AnnounceRequestData, AnnounceResponseData, AnnounceBypassData};
use actix_web::*;
use deadpool_redis::redis::Value;
use bendy::encoding::ToBencode;

#[get("/announce")]
pub async fn announce(
    web::Query(mut q): web::Query<AnnounceRequestData>,
    req: HttpRequest,
    data: web::Data<Context>,
) -> ProxyResult {
    let peer_ip = req.peer_addr().map(|addr| addr.ip());
    q.validation()?;
    q.fix_ip(peer_ip);

    let mut cxn = data.pool.get().await?;
    let cmd = q.generate_announce_cmd();
    let t: Vec<Value> = cmd.query_async(&mut cxn).await?;
    let response = AnnounceResponseData::from(t);
    let x = response.to_bencode()?;

    let req = AnnounceBypassData::from(q);
    let req = serde_qs::to_string(&req)?;
    let addr = format!("{}?{}", CONFIG.backend_announce_addr, req);
    let resp = reqwest::get(addr).await.map_err(|_| ProxyError::RequestError("bypass to backend failed"))?;
    if !resp.status().is_success() {
        return Err(ProxyError::RequestError("bypass to backend failed"));
    }

    Ok(HttpResponse::Ok().body(x))
}