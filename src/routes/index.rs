use worker::*;

const RESPONSE: &str = "All available routes\n
/               this page
/random         get a random quote
/quotes         get all quotes
/quotes/:id     get quote by id
/authors        get all authors
/authors/:id    get all authors by id/name/slug
/tags           get all tags
";

pub async fn handle_request(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok(RESPONSE)
}