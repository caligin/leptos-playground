#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use entirely_ssr_fns::{app::*, fileserv::file_and_error_handler};
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    #[allow(unused_variables)]
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    #[cfg(debug_assertions)]
    {
        log::info!("listening on http://{}", &addr);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    #[cfg(not(debug_assertions))]
    {
        lambda_http::run(app).await.unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
