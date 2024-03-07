use axum::{response::Html, routing::get, Router};
use std::sync::Arc;
use tera::Tera;

#[tokio::main]
async fn main() {
    // Initialize Tera
    let tera = Tera::new("templates/**/*.html").expect("Failed to initialize Tera");
    let tera = Arc::new(tera);

    // Initialize Axum app
    let app = Router::new()
        .route("/", get(handle_index))
        .route("/clicked", get(handle_clicked))
        .layer(axum::Extension(tera));

    // Start the server
    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn render_template(
    tera: &Arc<Tera>,
    template_name: &str,
    context: &tera::Context,
) -> Result<String, tera::Error> {
    let html = tera.render(template_name, context)?;

    Ok(html)
}

pub async fn handle_index(tera: axum::extract::Extension<Arc<Tera>>) -> Html<String> {
    let context = tera::Context::new();
    let html = render_template(&tera, "index.html", &context)
        .await
        .unwrap();

    Html(html)
}

pub async fn handle_clicked(tera: axum::extract::Extension<Arc<Tera>>) -> Html<String> {
    let context = tera::Context::new();
    let html = render_template(&tera, "clicked.html", &context)
        .await
        .unwrap();

    Html(html)
}
