use anyhow::anyhow;
// use anyhow::Result;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

enum DiagnosticSeverity {
    Error,
    Warning,
    None,
}

struct Backend {
    client: Client,
}

struct LintRule {
    name: String,
    message: String,
    severity: DiagnosticSeverity,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    println!("Starting server..");
    std::fs::write("testingserver", "Am I Launched?")
        .map_err(|e| anyhow!("Failed to wrtie File: {}", e))?;
    println!("file written successfully");

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
    // Err(anyhow::anyhow!("Not implemented"))

    Ok(())
}
