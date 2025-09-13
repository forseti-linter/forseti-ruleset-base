use forseti_sdk::core::{Diagnostic, Position, Range};
use forseti_sdk::ruleset::{Rule, RuleContext};

pub struct NoEmptyFilesRule;

impl Rule for NoEmptyFilesRule {
    fn id(&self) -> &'static str {
        "no-empty-files"
    }

    fn description(&self) -> &'static str {
        "Flags files that are completely empty or contain only whitespace characters"
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::Value::String("error".to_string())
    }

    fn check(&self, ctx: &mut RuleContext) {
        if ctx.text.trim().is_empty() {
            let diagnostic = Diagnostic {
                rule_id: self.id().to_string(),
                message: "File is empty or contains only whitespace".to_string(),
                severity: "error".to_string(),
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 0 },
                },
                code: None,
                suggest: None,
                docs_url: Some("https://forseti.dev/rules/no-empty-files".to_string()),
            };

            ctx.report(diagnostic);
        }
    }
}