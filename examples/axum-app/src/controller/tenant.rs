use crate::model::{Tenant, TenantColumn::*};
use std::time::Instant;
use harpiya_rs::{prelude::*, Request, Response, Result};

pub async fn new(mut req: Request) -> Result {
    let mut tenant = Tenant::new();
    let mut res = req.model_validation(&mut tenant).await?;
    let validation = tenant.check_constraints().await.extract(&req)?;
    if !validation.is_success() {
        reject!(req, validation);
    }

    tenant.insert().await.extract(&req)?;


    let data = json!({
        "method": req.request_method().as_ref(),
        "path": req.request_path(),
    });
    res.set_json_data(data);
    Ok(res.into())
}

pub async fn view(req: Request) -> Result {
    let tenant_id = req.parse_param("id")?;

    let db_query_start_time = Instant::now();
    let tenant = Tenant::fetch_by_id(&tenant_id).await.extract(&req)?;
    let db_query_duration = db_query_start_time.elapsed();

    let data = Map::data_entry(tenant);
    let mut res = Response::default().context(&req);
    res.record_server_timing("db", None, db_query_duration);
    res.set_json_data(data);
    Ok(res.into())
}

pub async fn stats(req: Request) -> Result {
    let query = QueryBuilder::<Tenant>::new()
        .aggregate(Aggregation::Count(Id, false), Some("num_tenants"))
        .limit(10)
        .build();
    let items = Tenant::aggregate::<Map>(&query).await.extract(&req)?;

    let mut res = Response::default().context(&req);
    res.set_json_data(Tenant::data_items(items));
    Ok(res.into())
}
