use forseti_sdk::core::{Diagnostic, LineIndex, Range};
use forseti_sdk::ruleset::{Rule, RuleContext};

pub struct MaxLineLengthRule;

impl Rule for MaxLineLengthRule {
    fn id(&self) -> &'static str {
        "max-line-length"
    }

    fn description(&self) -> &'static str {
        "Enforces maximum line length to improve code readability and maintainability"
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::Value::String("error".to_string())
    }

    fn check(&self, ctx: &mut RuleContext) {
        let limit = ctx
            .options
            .get("limit")
            .and_then(|v| v.as_u64())
            .unwrap_or(120) as usize;

        let line_index = LineIndex::new(ctx.text);
        let mut byte_offset = 0;

        for line in ctx.text.lines() {
            if line.chars().count() > limit {
                let start_pos = line_index.to_pos(byte_offset + limit);
                let end_pos = line_index.to_pos(byte_offset + line.len());

                let diagnostic = Diagnostic {
                    rule_id: self.id().to_string(),
                    message: format!("Line exceeds maximum length of {} characters", limit),
                    severity: "warn".to_string(),
                    range: Range {
                        start: start_pos,
                        end: end_pos,
                    },
                    code: None,
                    suggest: None,
                    docs_url: Some("https://forseti.dev/rules/max-line-length".to_string()),
                };

                ctx.report(diagnostic);
            }

            // Add 1 for the newline character
            byte_offset += line.len() + 1;
        }
    }
}
