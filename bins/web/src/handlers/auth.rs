use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use crate::AppContext;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    message: Option<&'a str>,
}

pub async fn login_get(_ctx: actix_web::web::Data<AppContext>, _req: HttpRequest
) -> actix_web::Result<impl Responder> {
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");
    let res = res.body((LoginTemplate { message: None })
                           .render().unwrap());
    Ok(res)
}

#[derive(Deserialize)]
pub struct LoginPostArgs {
    username: String,
    password: String,
}

pub async fn login_post(
    ctx: actix_web::web::Data<AppContext>,
    req: HttpRequest,
    form: actix_web::web::Form<LoginPostArgs>,
) -> actix_web::Result<impl Responder> {
    let auth_token = ctx.auth.login(&form.username, &form.password);

    if auth_token.is_some() {
        info!("peer={:?} Logged in", req.peer_addr());
        let mut res = HttpResponse::SeeOther(); // 303
        if let Err(e) = ctx.sessions.login(&mut res) {
            error!("Error calling Sessions::login: {}", e);
            return Err(().into());
        }
        res.header(actix_web::http::header::LOCATION, "/");
        Ok(res.finish())
    } else {
        assert!(auth_token.is_none());

        info!("peer={:?} Bad username or password", req.peer_addr());

        let mut res = HttpResponse::Unauthorized(); // 401
        res.content_type("text/html");
        let res = res.body((LoginTemplate { message: Some("Bad username or password") })
                           .render().unwrap());
        Ok(res)
    }
}

pub async fn logout_get(ctx: actix_web::web::Data<AppContext>, req: HttpRequest
) -> actix_web::Result<impl Responder> {
    let mut res = HttpResponse::Ok();
    res.content_type("text/html");

    ctx.sessions.logout(&req, &mut res);
    let res = res.body((LoginTemplate { message: Some("Logged out") })
                           .render().unwrap());
    Ok(res)
}
