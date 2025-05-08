use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use crate::compiler::parser::ASTNode;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;
use tokio::net::TcpListener;

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
                                ASTNode::StringLiteral { value } => value.clone(),
                                _ => "Internal Error: Invalid return type".to_string(),
                            }
                        } else {
                            "Internal Error: No return statement".to_string()
                        }
                    } else {
                        "Internal Error: Invalid body".to_string()
                    };

                    let response = response.clone();
                    router = router.route(&path, match method.as_str() {
                        "get" => get(move || async move { response.clone() }),
                        _ => get(|| async { "Method not supported" }),
                    });
                }
            }
        }

        // Add a default health check endpoint only if one isn't already defined
        if !has_health_check {
            router = router.route("/health", get(|| async { "OK" }));
        }

        router
    }
}

// Helper function to extract parameters from the AST
fn extract_params(ast: &ASTNode) -> Vec<String> {
    if let ASTNode::Endpoint { params, .. } = ast {
        params.iter().map(|p| p.name.clone()).collect()
    } else {
        vec![]
    }
} 