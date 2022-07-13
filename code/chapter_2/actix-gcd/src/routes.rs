use crate::computing;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GcdParameters {
    n: u64,
    m: u64,
}

pub fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method=post>
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit"> Compute GCD </button>
            </form>
        "#,
    )
}

pub fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }
    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        computing::gcd(form.n, form.m)
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}
