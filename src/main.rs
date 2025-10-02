use std::sync::Arc;
use anyhow::Result;
use rmcp::{
    ServerHandler, ServiceExt,
    model::*,
    service::{RequestContext, RoleServer},
    transport::stdio,
};
use tracing_subscriber::{self, EnvFilter};

#[derive(Clone, Debug, Default)]
pub struct ParinferServer;

impl ServerHandler for ParinferServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation {
                name: "parinfer-server".into(),
                version: "0.1.0".into(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "This server provides a tool to fix Clojure code parentheses based on indentation. \
                 When compilation fails due to mismatched, missing, or extra closing delimiters (parentheses, brackets, braces), \
                 use the 'fix_parens' tool instead of trying to fix them manually. \
                 The tool is particularly helpful for larger code blocks where delimiter balancing is complex.".into()
            ),
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, ErrorData> {
        match request.name.as_ref() {
            "fix_parens" => {
                let code = request
                    .arguments
                    .as_ref()
                    .and_then(|args| args.get("code"))
                    .and_then(|c| c.as_str())
                    .ok_or_else(|| {
                        ErrorData::new(
                            ErrorCode::INVALID_PARAMS,
                            "Missing required parameter 'code'".to_string(),
                            None,
                        )
                    })?;

                let options = parinfer_rust::types::Options {
                    cursor_x: None,
                    cursor_line: None,
                    prev_cursor_x: None,
                    prev_cursor_line: None,
                    prev_text: None,
                    selection_start_line: None,
                    changes: vec![],
                    comment_char: ';',
                    string_delimiters: vec!["\"".to_string()],
                    lisp_vline_symbols: false,
                    lisp_block_comments: false,
                    guile_block_comments: false,
                    scheme_sexp_comments: false,
                    janet_long_strings: false,
                    hy_bracket_strings: false,
                };

                let answer = parinfer_rust::parinfer::indent_mode(code, &options);

                if answer.success {
                    Ok(CallToolResult::success(vec![Content::text(
                        answer.text.to_string()
                    )]))
                } else {
                    Err(ErrorData::new(
                        ErrorCode::INTERNAL_ERROR,
                        format!("Parinfer error: {:?}", answer.error),
                        None,
                    ))
                }
            }
            _ => Err(ErrorData::new(
                ErrorCode::METHOD_NOT_FOUND,
                format!("Unknown tool: {}", request.name),
                None,
            )),
        }
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(ListToolsResult {
            tools: vec![Tool {
                name: "fix_parens".into(),
                title: Some("Fix Clojure Parentheses".into()),
                description: Some(
                    "Automatically fixes closing delimiters in Clojure code: missing parens, wrong types (e.g., ')' instead of ']' ), extra delimiters, or mismatched nesting. \
                     IMPORTANT: When compilation fails with parenthesis/bracket/brace errors, use this tool rather than attempting manual fixes. \
                     It infers correct delimiters from indentation, which is more reliable for complex code blocks as AIs are well trained on indeted code, but less oso on lisp-like code. \
                     Input: Clojure code with correct indentation (indentation determines the intended structure). \
                     Output: Same code with all closing delimiters (, [, { properly balanced."
                        .into(),
                ),
                input_schema: Arc::new(
                    serde_json::from_value(serde_json::json!({
                        "type": "object",
                        "properties": {
                            "code": {
                                "type": "string",
                                "description": "Clojure / ClojureScript code with correct indentation but potentially incorrect closing delimiters. The tool infers proper delimiters from indentation."
                            }
                        },
                        "required": ["code"]
                    }))
                    .unwrap(),
                ),
                output_schema: None,
                annotations: None,
                icons: None,
            }],
            next_cursor: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Parinfer MCP Server");

    let service = ParinferServer.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("Serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
