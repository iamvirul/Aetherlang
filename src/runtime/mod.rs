use axum::{
    routing::{get, post, put, delete},
    Router,
    response::Json,
    extract::Query,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use crate::compiler::parser::{ASTNode, Parameter}; // Added Parameter
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
            for endpoint_ast in endpoints {
                if let ASTNode::Endpoint { path, method, params, body, .. } = endpoint_ast {
                    if path == "/health" {
                        has_health_check = true;
                    }

                    let current_path = path.clone();
                    // Clone necessary AST parts for the handler
                    let endpoint_params_ast: Vec<Parameter> = params.clone(); 
                    let endpoint_body_ast: Box<ASTNode> = body.clone();

                    router = router.route(&current_path, match method.as_str() {
                        "get" => {
                            get(move |query_params: Query<HashMap<String, String>>| {
                                let captured_endpoint_params = endpoint_params_ast.clone();
                                let captured_body = endpoint_body_ast.clone();
                                async move {
                                    if let ASTNode::Block { statements } = *captured_body {
                                        if let Some(ASTNode::ReturnStatement { expression }) = statements.first() {
                                            match &**expression {
                                                ASTNode::StringLiteral { value } => {
                                                    let mut processed_value = value.clone();
                                                    for ast_param in captured_endpoint_params {
                                                        if let Some(param_val) = query_params.0.get(&ast_param.name) {
                                                            // Construct the pattern \(name) directly
                                                            let slash = "\\"; // Literal backslash
                                                            let open_paren = "(";
                                                            let close_paren = ")";
                                                            let pattern_to_search = format!("{}{}{}{}", slash, open_paren, ast_param.name, close_paren);
                                                            processed_value = processed_value.replace(&pattern_to_search, param_val);
                                                        } else {
                                                            // Parameter not found in query
                                                            return Json(json!({ "error": format!("Missing required parameter: {}", ast_param.name) }));
                                                        }
                                                    }
                                                    Json(json!({ "data": processed_value }))
                                                }
                                                ASTNode::Identifier { name } => {
                                                    if let Some(param_val) = query_params.0.get(name) {
                                                        Json(json!({ "data": param_val }))
                                                    } else {
                                                        Json(json!({ "error": format!("Identifier '{}' not found in query parameters", name) }))
                                                    }
                                                }
                                                _ => Json(json!({ "error": "Invalid return type for dynamic processing" })),
                                            }
                                        } else {
                                            Json(json!({ "error": "No return statement in endpoint body" }))
                                        }
                                    } else {
                                        Json(json!({ "error": "Invalid endpoint body structure" }))
                                    }
                                }
                            })
                        },
                        "post" => post(|| async { Json(json!({ "message": "POST not fully implemented for dynamic params yet" })) }),
                        "put" => put(|| async { Json(json!({ "message": "PUT not fully implemented for dynamic params yet" })) }),
                        "delete" => delete(|| async { Json(json!({ "message": "DELETE not fully implemented for dynamic params yet" })) }),
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