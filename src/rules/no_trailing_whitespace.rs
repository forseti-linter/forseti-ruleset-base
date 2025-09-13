use forseti_sdk::core::{Diagnostic, LineIndex, Range};
use forseti_sdk::ruleset::{Rule, RuleContext};

pub struct NoTrailingWhitespaceRule;

impl Rule for NoTrailingWhitespaceRule {
    fn id(&self) -> &'static str {
        "no-trailing-whitespace"
    }

    fn description(&self) -> &'static str {
        "Detects and flags trailing whitespace (spaces and tabs) at the end of lines"
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::Value::String("error".to_string())
    }

    fn check(&self, ctx: &mut RuleContext) {
        let line_index = LineIndex::new(ctx.text);
        
        for (line_num, line) in ctx.text.lines().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                // Find the trailing whitespace
                let trimmed = line.trim_end();
                let start_offset = line_index.to_pos(
                    ctx.text.lines().take(line_num).map(|l| l.len() + 1).sum::<usize>() + trimmed.len()
                );
                let end_offset = line_index.to_pos(
                    ctx.text.lines().take(line_num + 1).map(|l| l.len() + 1).sum::<usize>() - 1
                );

                let diagnostic = Diagnostic {
                    rule_id: self.id().to_string(),
                    message: "Trailing whitespace found".to_string(),
                    severity: "warn".to_string(),
                    range: Range {
                        start: start_offset,
                        end: end_offset,
                    },
                    code: None,
                    suggest: None,
                    docs_url: Some("https://forseti.dev/rules/no-trailing-whitespace".to_string()),
                };

                ctx.report(diagnostic);
            }
        }
    }
}
