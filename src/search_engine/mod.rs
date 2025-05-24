mod applications;

#[derive(Debug, Clone, PartialEq)]
pub struct Action {
    pub name: String,
    pub description: String,
}

#[derive(Clone)]
pub struct CustomActionRule {
    pub keyword: String,
    pub action_to_return: Action,
}

pub struct SearchEngine {
    custom_action_rules: Vec<CustomActionRule>,
}

pub trait CustomAction {
    fn register(&self) -> CustomActionRule;
    // The action for this action. e.g for applications
    // is this the function which will execute whatever
    // the actions is doing
    fn action(&self);
}

impl SearchEngine {
    pub fn new() -> Self {
        let mut se = SearchEngine {
            custom_action_rules: Vec::new(),
        };

        se.register_custom_action_rule(CustomActionRule {
            keyword: "switch song".to_string(),
            action_to_return: Action {
                name: "Switch Song".to_string(),
                description: "Placeholder: Command to switch the current song.".to_string(),
            },
        });

        se
    }

    pub fn register_custom_action_rule(&mut self, rule: CustomActionRule) {
        self.custom_action_rules.push(rule);
    }

    pub fn search(&self, query: &str) -> Vec<Action> {
        let mut results = Vec::new();
        let lower_query = query.trim().to_lowercase();

        if lower_query.is_empty() {
            return results;
        }

        let parts: Vec<&str> = lower_query.split_whitespace().collect();
        if parts.len() == 3 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<f64>(), parts[2].parse::<f64>()) {
                let op = parts[1];
                let calculation_result = match op {
                    "/" if num2 != 0.0 => Some(num1 / num2),
                    "*" | "x" => Some(num1 * num2),
                    "+" => Some(num1 + num2),
                    "-" => Some(num1 - num2),
                    _ => None,
                };
                if let Some(res) = calculation_result {
                    results.push(Action {
                        name: format!("Calculate: {}", query),
                        description: format!("Result: {}", res),
                    });
                }
            }
        }

        for rule in &self.custom_action_rules {
            if lower_query.contains(&rule.keyword.to_lowercase()) {
                if !results.iter().any(|r| r.name == rule.action_to_return.name) {
                    results.push(rule.action_to_return.clone());
                }
            }
        }

        results
    }
}
