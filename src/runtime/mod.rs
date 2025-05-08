use axum::{
    routing::{get, post, put, delete},
    Router,
    response::Json,
};
use std::net::SocketAddr;
use crate::compiler::parser::ASTNode;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tokio::net::TcpListener;
use serde_json::json;

pub struct Runtime {
    ast: ASTNode,
    port: u16,
}

impl Runtime {
    pub fn new(ast: ASTNode, port: u16) -> Self {
        Runtime { ast, port }
    }

    pub async fn start(&self) -> Result<(), String> {
        // Initialize the tracing subscriber
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_target(false)
            .with_thread_ids(false)
            .with_file(false)
            .with_line_number(false)
            .compact()
            .init();

        let app = self.build_router()
            .layer(ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()));

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        info!("🚀 Aether service running on http://{}", addr);
        info!("Press Ctrl+C to stop the server");

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| format!("Failed to bind: {}", e))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| format!("Server error: {}", e))?;

        Ok(())
    }

    fn build_router(&self) -> Router {
        let mut router = Router::new();
        let mut has_health_check = false;

        if let ASTNode::Service { endpoints, .. } = &self.ast {
            for endpoint in endpoints {
                if let ASTNode::Endpoint { path, method, body, .. } = endpoint {
                    if path == "/health" {
                        has_health_check = true;
                    }

                    let path = path.clone();
                    let response = if let ASTNode::Block { statements } = &**body {
                        if let Some(ASTNode::ReturnStatement { expression }) = statements.first() {
                            match &**expression {
                                ASTNode::StringLiteral { value } => json!({ "data": value }),
                                ASTNode::Identifier { name } => json!({ "data": name }),
                                _ => json!({ "error": "Invalid return type" }),
                            }
                        } else {
                            json!({ "error": "No return statement" })
                        }
                    } else {
                        json!({ "error": "Invalid body" })
                    };

                    let response = response.clone();
                    router = router.route(&path, match method.as_str() {
                        "get" => get(move || async move { Json(response.clone()) }),
                        "post" => post(move || async move { Json(response.clone()) }),
                        "put" => put(move || async move { Json(response.clone()) }),
                        "delete" => delete(move || async move { Json(response.clone()) }),
                        _ => get(|| async { Json(json!({ "error": "Method not supported" })) }),
                    });
                }
            }
        }

        // Add default endpoints
        if !has_health_check {
            router = router.route("/health", get(|| async { 
                Json(json!({ "status": "OK", "timestamp": chrono::Utc::now().to_rfc3339() }))
            }));
        }

        // Add system endpoints
        router = router
            .route("/system/info", get(|| async { 
                Json(json!({
                    "version": env!("CARGO_PKG_VERSION"),
                    "name": env!("CARGO_PKG_NAME"),
                    "authors": env!("CARGO_PKG_AUTHORS"),
                    "description": env!("CARGO_PKG_DESCRIPTION")
                }))
            }))
            .route("/system/routes", get(|| async {
                Json(json!({
                    "available_routes": [
                        "/health",
                        "/system/info",
                        "/system/routes"
                    ]
                }))
            }));

        router
    }
} 