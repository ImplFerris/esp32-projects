use core::{include_str, sync::atomic::Ordering};
use embassy_net::Stack;
use embassy_time::Duration;
use heapless::String;
use picoserve::{
    response::{File, IntoResponse},
    routing, AppBuilder, AppRouter, Router,
};

pub const WEB_TASK_POOL_SIZE: usize = 2;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
pub async fn web_task(
    id: usize,
    stack: Stack<'static>,
    app: &'static AppRouter<Application>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        app,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

pub struct WebApp {
    pub app: &'static Router<<Application as AppBuilder>::PathRouter>,
    pub config: &'static picoserve::Config<Duration>,
}

impl Default for WebApp {
    fn default() -> Self {
        let app = picoserve::make_static!(AppRouter<Application>, Application.build_app());

        let config = picoserve::make_static!(
            picoserve::Config<Duration>,
            picoserve::Config::new(picoserve::Timeouts {
                start_read_request: Some(Duration::from_secs(5)),
                read_request: Some(Duration::from_secs(1)),
                write: Some(Duration::from_secs(1)),
            })
            .keep_connection_alive()
        );

        Self { app, config }
    }
}

pub struct Application;

impl AppBuilder for Application {
    type PathRouter = impl routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        picoserve::Router::new()
            .route(
                "/",
                routing::get_service(File::html(include_str!("index.html"))),
            )
            .route("/led", routing::post(led_handler))
    }
}

async fn led_handler(input: picoserve::extract::Json<LedRequest, 0>) -> impl IntoResponse {
    if input.0.action == "on" {
        crate::led::LED_STATE.store(true, Ordering::Relaxed);
    } else {
        crate::led::LED_STATE.store(false, Ordering::Relaxed);
    }
    picoserve::response::Json(LedResponse { success: true })
}

#[derive(serde::Serialize)]
struct LedResponse {
    success: bool,
}

#[derive(serde::Deserialize)]
struct LedRequest {
    action: String<5>,
}
