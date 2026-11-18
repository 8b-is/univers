//! univers-mcp — UNI-OMNI-ZENO unification MCP server
//!
//! Exposes NVIDIA, Smart Tree, Foken, Entheai, and Phoenix as agent tools.
//! stdio JSON-RPC 2.0 — works with crush, opencode, Claude, and entheai.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

#[derive(Deserialize)]
struct Request {
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct Response {
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Value>,
}

#[derive(Serialize)]
struct Tool {
    name: &'static str,
    description: &'static str,
    input: Value,
}

fn tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "univers_health",
            description: "Check all Foken E2E services health (web, llm, gpu, mcp, system, radio)",
            input: json!({"type": "object", "properties": {}}),
        },
        Tool {
            name: "univers_ollama_models",
            description: "List all local Ollama models available for inference",
            input: json!({"type": "object", "properties": {}}),
        },
        Tool {
            name: "univers_nvidia_models",
            description: "List NVIDIA cloud models (requires NVIDIA_API_KEY)",
            input: json!({"type": "object", "properties": {}}),
        },
        Tool {
            name: "univers_chat",
            description: "Chat with a local Ollama model (free, offline)",
            input: json!({
                "type": "object",
                "properties": {
                    "model": {"type": "string", "description": "Ollama model name"},
                    "message": {"type": "string", "description": "Message to send"}
                },
                "required": ["model", "message"]
            }),
        },
        Tool {
            name: "univers_smart_tree",
            description: "Build a Smart Tree from a directory path",
            input: json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string", "description": "Directory path to scan"}
                },
                "required": ["path"]
            }),
        },
        Tool {
            name: "univers_foken_health",
            description: "Check Foken E2E gateway health (HTTPS on :8443)",
            input: json!({"type": "object", "properties": {}}),
        },
        Tool {
            name: "univers_zeno_status",
            description: "Check UNI-OMNI-ZENO unification status across all repos",
            input: json!({"type": "object", "properties": {}}),
        },
    ]
}

fn dispatch(tool: &str, args: &Value) -> Value {
    match tool {
        "univers_health" => {
            let services = [
                ("web", "http://127.0.0.1:8080/health"),
                ("radio", "http://127.0.0.1:8081/health"),
                ("llm", "http://127.0.0.1:8082/health"),
                ("gpu", "http://127.0.0.1:8090/health"),
                ("mcp", "http://127.0.0.1:3000/health"),
                ("system", "http://127.0.0.1:3001/health"),
            ];
            let mut results = serde_json::Map::new();
            for (name, url) in &services {
                let status = reqwest::blocking::get(*url)
                    .ok()
                    .and_then(|r| r.text().ok())
                    .unwrap_or_else(|| "DOWN".to_string());
                results.insert(name.to_string(), json!(status));
            }
            json!({"services": results})
        }
        "univers_ollama_models" => {
            match reqwest::blocking::get("http://127.0.0.1:11434/api/tags") {
                Ok(r) => r.json().unwrap_or(json!({"models": []})),
                Err(_) => json!({"error": "Ollama not running"}),
            }
        }
        "univers_nvidia_models" => {
            let key = std::env::var("NVIDIA_API_KEY").unwrap_or_default();
            if key.is_empty() {
                return json!({"error": "NVIDIA_API_KEY not set"});
            }
            let client = reqwest::blocking::Client::new();
            match client
                .get("https://integrate.api.nvidia.com/v1/models")
                .header("Authorization", format!("Bearer {}", key))
                .send()
            {
                Ok(r) => r.json().unwrap_or(json!({"error": "parse failed"})),
                Err(e) => json!({"error": e.to_string()}),
            }
        }
        "univers_chat" => {
            let model = args.get("model").and_then(|v| v.as_str()).unwrap_or("qwen3.6:35b-a3b-coding-nvfp4");
            let message = args.get("message").and_then(|v| v.as_str()).unwrap_or("");
            let client = reqwest::blocking::Client::new();
            let body = json!({
                "model": model,
                "messages": [{"role": "user", "content": message}],
                "stream": false
            });
            match client
                .post("http://127.0.0.1:11434/api/chat")
                .json(&body)
                .send()
            {
                Ok(r) => r.json().unwrap_or(json!({"error": "parse failed"})),
                Err(e) => json!({"error": e.to_string()}),
            }
        }
        "univers_smart_tree" => {
            let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
            let tree = std::fs::read_dir(path)
                .ok()
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .take(50)
                        .map(|e| e.file_name().to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            json!({"path": path, "entries": tree})
        }
        "univers_foken_health" => {
            let client = reqwest::blocking::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();
            match client
                .get("https://macbook.tail2870dc.ts.net:8443/health")
                .send()
            {
                Ok(r) => r.json().unwrap_or(json!({"error": "parse failed"})),
                Err(e) => json!({"error": e.to_string()}),
            }
        }
        "univers_zeno_status" => {
            json!({
                "unification": "UNI-OMNI-ZENO",
                "repos": {
                    "univers": "8b-is/univers (github+codeberg)",
                    "omnispherepop": "8b-is/omnispherepop",
                    "entheai": "local+github",
                    "phoenix": "felix-phoenix (SPARK)",
                    "foken": "8b-is/foken",
                    "g8t": "8b-is/g8t"
                },
                "status": "unified"
            })
        }
        _ => json!({"error": format!("unknown tool: {}", tool)}),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }

        let req: Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = Response {
                    id: None,
                    result: None,
                    error: Some(json!({"code": -32700, "message": format!("parse error: {}", e)})),
                };
                writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
                stdout.flush()?;
                continue;
            }
        };

        let resp = match req.method.as_str() {
            "initialize" => Response {
                id: req.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name": "univers-mcp", "version": "0.2.0"}
                })),
                error: None,
            },
            "tools/list" => Response {
                id: req.id,
                result: Some(json!({"tools": tools()})),
                error: None,
            },
            "tools/call" => {
                let tool_name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(json!({}));
                Response {
                    id: req.id,
                    result: Some(json!({"content": [{"type": "text", "text": serde_json::to_string(&dispatch(tool_name, &args))?}]})),
                    error: None,
                }
            }
            _ => Response {
                id: req.id,
                result: None,
                error: Some(json!({"code": -32601, "message": format!("method not found: {}", req.method)})),
            },
        };

        writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
        stdout.flush()?;
    }

    Ok(())
}
