use anyhow::anyhow;
use std::collections::HashMap;
use tokio::sync::Mutex;
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
    document_map: Mutex<HashMap<String, String>>,
}

struct LintRule {
    name: String,
    message: String,
    severity: DiagnosticSeverity,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client: client,
            document_map: Mutex::new(HashMap::new()),
        }
    }

    async fn analyze_document(&self, uri: &str) {
        let documents = self.document_map.lock().await;

        // todo: activate linter and show diagnostics
    }

    fn get_lint_rules(&self) -> Vec<LintRule> {
        vec![LintRule {
            name: "no-var".to_string(),
            message: "Use 'let' or 'const' instead of 'var'".to_string(),
            severity: DiagnosticSeverity::Warning,
        }]
    }

    async fn lint(&self, content: &str) -> Vec<Diagnostic> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            if line.contains("var ") {
                let start_char = line.find("var ").unwrap_or(0) as u32;

                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_idx as u32,
                            character: start_char,
                        },
                        end: Position {
                            line: line_idx as u32,
                            character: start_char + 3,
                        },
                    },
                    severity: Some(tower_lsp::lsp_types::DiagnosticSeverity::WARNING),
                    message: "Use 'let' or 'const' instead of 'var'".to_string(),
                    source: Some("js-linter".to_string()),
                    ..Default::default()
                });
            }
        }

        diagnostics
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;

        // store documents
        let mut documents = self.document_map.lock().await;
        documents.insert(uri.clone(), text);

        // analyze document and publish diagnostics
        self.analyze_document(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();

        if let Some(change) = params.content_changes.last() {
            let mut documents = self.document_map.lock().await;
            documents.insert(uri.clone(), change.text.clone());

            drop(documents);

            // re-analyze document
            self.analyze_document(&uri).await;
        }
    }

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("You're hovering!".to_string())),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    println!("Starting server..");
    std::fs::write("testingserver", "Am I Launched?")
        .map_err(|e| anyhow!("Failed to wrtie File: {}", e))?;
    println!("file written successfully");

    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
    // Err(anyhow::anyhow!("Not implemented"))

    Ok(())
}
