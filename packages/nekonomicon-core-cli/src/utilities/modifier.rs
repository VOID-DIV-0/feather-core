pub enum ModifierKind {
    Flag(String),
    KeyValue { key: String, value: String },
}

pub struct Modifier {
    pub mode: ModifierMode,
    pub kind: ModifierKind,
}

pub enum ModifierMode {
    With,
    Without,
}

pub fn handler(args: &[String]) -> Vec<Modifier> {
    let mut modifiers = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let mode = match args[i].as_str() {
            "with" => ModifierMode::With,
            "without" => ModifierMode::Without,
            _ => {
                i += 1;
                continue;
            }
        };
        if i + 1 < args.len() {
            let key = args[i + 1].clone();
            if i + 2 < args.len() && args[i + 2].starts_with('\'') && args[i + 2].ends_with('\'') {
                let value = args[i + 2].trim_matches('\'').to_string();
                modifiers.push(Modifier {
                    mode,
                    kind: ModifierKind::KeyValue { key, value },
                });
                i += 3;
            } else {
                modifiers.push(Modifier {
                    mode,
                    kind: ModifierKind::Flag(key),
                });
                i += 2;
            }
        } else {
            i += 1;
        }
    }
    modifiers
}
