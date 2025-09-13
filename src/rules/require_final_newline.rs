use forseti_sdk::core::{Diagnostic, LineIndex, Range, Fix, SuggestFix};
use forseti_sdk::ruleset::{Rule, RuleContext};

pub struct RequireFinalNewlineRule;

impl Rule for RequireFinalNewlineRule {
    fn id(&self) -> &'static str {
        "require-final-newline"
    }

    fn description(&self) -> &'static str {
        "Ensures files end with a newline character for better POSIX compliance and tool compatibility"
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::Value::String("error".to_string())
    }

    fn check(&self, ctx: &mut RuleContext) {
        if !ctx.text.is_empty() && !ctx.text.ends_with('\n') {
            let line_index = LineIndex::new(ctx.text);
            let end_pos = line_index.to_pos(ctx.text.len());

            let diagnostic = Diagnostic {
                rule_id: self.id().to_string(),
                message: "File must end with a newline character".to_string(),
                severity: "warn".to_string(),
                range: Range {
                    start: end_pos,
                    end: end_pos,
                },
                code: None,
                suggest: Some(vec![
                    SuggestFix {
                        title: "Add final newline".to_string(),
                        fix: Some(Fix {
                            range: Range {
                                start: end_pos,
                                end: end_pos,
                            },
                            text: "\n".to_string(),
                        }),
                    }
                ]),
                docs_url: Some("https://forseti.dev/rules/require-final-newline".to_string()),
            };

            ctx.report(diagnostic);
        }
    }
}
