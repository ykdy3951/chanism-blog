use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower_http::services::ServeDir;
use crate::app::App;

pub async fn init_app(configuration_path: Option<&str>) {
    tracing_subscriber::fmt()
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let conf = get_configuration(configuration_path).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(App);

    let leptos_options = conf.leptos_options;
    let serve_dir = ServeDir::new(&leptos_options.site_root)
        .append_index_html_on_directories(false);

    let app = axum::Router::new()
        .leptos_routes(&leptos_options, routes, || view! { <App /> })
        .fallback_service(serve_dir)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new().level(tracing::Level::INFO),
                )
                .on_failure(
                    tower_http::trace::DefaultOnFailure::new().level(tracing::Level::ERROR),
                ),
        )
        // .layer(axum::middleware::from_fn(crate::auth::auth_middleware))
        .with_state(leptos_options);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address");
    logging::log!("Listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to serve app");
}