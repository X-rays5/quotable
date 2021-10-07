use worker::*;

mod utils;
mod data;
mod routes;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/random", routes::random::handle_request)
        .get_async("/quotes", routes::quotes::handle_request)
        .get_async("/quotes/:id", routes::quotes::handle_request_specific)
        .get_async("/authors", routes::authors::handle_request)
        .get_async("/authors/:id", routes::authors::handle_request_specific)
        .get_async("/tags", routes::tags::handle_request)
        .run(req, env)
        .await
}
