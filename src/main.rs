use anyhow::Result;
use forseti_sdk::core::{ConfigSetting, ConfigType, FileContext, PreprocessingContext, RulesetCapabilities};
use forseti_sdk::ruleset::{Ruleset, RulesetOptions, RulesetServer};
use serde_json::json;
use std::collections::HashMap;

mod rules;
use rules::*;

struct BaseRuleset;

impl BaseRuleset {
    fn get_config_settings(&self) -> Vec<ConfigSetting> {
        vec![
            ConfigSetting {
                name: "max-line-length".to_string(),
                description: "Maximum allowed line length in characters".to_string(),
                setting_type: ConfigType::Integer,
                default: json!(80),
                required: false,
                min: Some(10.0),
                max: Some(500.0),
                allowed_values: None,
            },
            // Add more meaningful configuration settings here
            // Rule enable/disable settings will be auto-injected by the SDK
        ]
    }
}

impl RulesetOptions for BaseRuleset {

    fn get_capabilities(&self) -> RulesetCapabilities {
        RulesetCapabilities {
            ruleset_id: "base".to_string(),
            version: "0.1.0".to_string(),
            file_patterns: vec!["*".to_string()], // Matches all files
            max_file_size: Some(10 * 1024 * 1024), // 10MB limit
            annotation_prefixes: vec![
                "#".to_string(),    // Common in many languages (Python, Bash, YAML, etc.)
                "//".to_string(),   // C-style comments (JS, Java, C++, etc.)
                "--".to_string(),   // SQL, Lua, Haskell
                "<!--".to_string(), // HTML/XML
            ],
            rules: vec![], // Will be populated by the server
            default_config: self.get_default_config(),
            config_settings: self.get_config_settings(),
        }
    }

    fn preprocess_files(&self, file_uris: &[String]) -> Result<PreprocessingContext> {
        let mut files = Vec::new();
        let mut global_context = HashMap::new();

        // Base ruleset: minimal preprocessing, no content loading
        // Just provide file metadata and let rules load content as needed
        for uri in file_uris {
            let mut context = HashMap::new();

            // Only gather lightweight file metadata
            if uri.starts_with("file://") {
                let path = uri.strip_prefix("file://").unwrap_or(uri);
                if let Ok(metadata) = std::fs::metadata(path) {
                    context.insert("file_size".to_string(), json!(metadata.len()));
                    context.insert("is_file".to_string(), json!(metadata.is_file()));
                }
                // Infer file type from extension
                if let Some(ext) = std::path::Path::new(path).extension() {
                    context.insert("extension".to_string(), json!(ext.to_string_lossy()));
                }
            }

            files.push(FileContext {
                uri: uri.clone(),
                content: String::new(), // Empty - rules will load content themselves
                language: infer_language(uri),
                context,
            });
        }

        // Global context with basic stats
        global_context.insert("total_files".to_string(), json!(files.len()));
        global_context.insert("ruleset_type".to_string(), json!("text-based"));

        Ok(PreprocessingContext {
            ruleset_id: "base".to_string(),
            files,
            global_context,
        })
    }

    fn create_ruleset(&self) -> Ruleset {
        Ruleset::new("base")
            .with_rule(Box::new(NoTrailingWhitespaceRule))
            .with_rule(Box::new(MaxLineLengthRule))
            .with_rule(Box::new(NoEmptyFilesRule))
            .with_rule(Box::new(RequireFinalNewlineRule))
    }
}

fn infer_language(uri: &str) -> Option<String> {
    let path = if uri.starts_with("file://") {
        uri.strip_prefix("file://").unwrap_or(uri)
    } else {
        uri
    };

    match std::path::Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
    {
        Some("txt") => Some("text".to_string()),
        Some("md") => Some("markdown".to_string()),
        Some("json") => Some("json".to_string()),
        Some("toml") => Some("toml".to_string()),
        Some("yaml") | Some("yml") => Some("yaml".to_string()),
        _ => None,
    }
}

fn main() -> Result<()> {
    let mut server = RulesetServer::new(Box::new(BaseRuleset));
    server.run_stdio()
}
