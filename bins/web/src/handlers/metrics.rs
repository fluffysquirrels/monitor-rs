use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::AppContext;
use monitor::metric_store::Metric;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    metrics: Vec<Metric>,
}

pub async fn metrics_get(ctx: actix_web::web::Data<AppContext>, req: HttpRequest
) -> actix_web::Result<impl Responder> {
    // If not logged in, redirect to "/login".
    let session = ctx.sessions.get_with_req(&req);
    if session.is_none() {
        let mut res = HttpResponse::SeeOther(); // 303
        res.header(actix_web::http::header::LOCATION, "/login");
        return Ok(res.finish())
    }

    // Authenticated.
    let metrics = ctx.metric_store.lock().unwrap().query_all();
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");
    let res = res.body((IndexTemplate {
        metrics,
    }).render().unwrap());
    Ok(res)
}
