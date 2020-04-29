use std::path::Path;

use actix_web::web::Json;
use actix_web::{HttpRequest, HttpResponse};
use filecoin_proofs_api::seal;
use log::trace;

use crate::seal_data::*;

pub async fn clear_cache(_req: HttpRequest, data: Json<ClearCacheData>) -> HttpResponse {
    trace!("clear_cache");

    let r = seal::clear_cache(data.sector_size, Path::new(&data.cache_path));

    HttpResponse::Ok().json(r.map_err(|e| format!("{:?}", e)))
}
