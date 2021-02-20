use lambda_http::{handler, lambda, Request, Context, Response, Body, IntoResponse};
use maud::html;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(handle_request)).await?;
    Ok(())
}

async fn handle_request(request: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let http_method = format!("{}", request.method());
    let url = format!("{}", request.uri());
    let markup = html! {
        h1 { "Rust HTTP Server" }
        h4 { "Request Info:" }
        p { (http_method) " " (url) }
        h4 { "Headers:" }
        ul {
            @for header in request.headers() {
                li { (format!("{:?}", header)) }
           }
        }
    };

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::from(markup.into_string()))
        .unwrap()
    )
}
