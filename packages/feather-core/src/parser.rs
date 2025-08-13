use crate::{token::{Token, TokenType}, tokenizer};

/// Abstract Syntax Tree (AST) representation for the Feather language.

#[derive(Debug)]
pub enum AbstractTreeNode {
    Instruction(InstructionNode),
    Modifier(ModifierNode),
    Literal(String),
    Variable(String),
}

#[derive(Debug)]
pub struct InstructionNode {
    pub name: String,
    pub args: Vec<AbstractTreeNode>,
}

#[derive(Debug)]
pub struct ModifierNode {
    pub kind: ModifierKind,
    pub args: Vec<AbstractTreeNode>,
}

#[derive(Debug)]
pub enum ModifierKind {
    As,
    With,
    Without,
    Unknown,
}

pub fn parser(input: &str) -> Vec<AbstractTreeNode> {
    let tokens = tokenizer::tokenize(input);
    let mut nodes = Vec::new();
    let mut token_iterator = 0;

    while token_iterator < tokens.len() {
        let token = &tokens[token_iterator];

        match token.kind {
            TokenType::Instruction => {
                // For now, only support 'say' with one literal argument
                if let Some(next) = tokens.get(token_iterator + 1) {
                    nodes.push(AbstractTreeNode::Instruction(InstructionNode {
                        name: token.value.clone(),
                        args: vec![AbstractTreeNode::Literal(next.value.clone())],
                    }));
                    token_iterator += 2;
                    continue;
                }
            }
            TokenType::Modifier => {
                // Parse modifier with one literal argument
                if let Some(next) = tokens.get(token_iterator + 1) {
                    let kind = match token.value.as_str() {
                        "as" => ModifierKind::As,
                        "with" => ModifierKind::With,
                        "without" => ModifierKind::Without,
                        _ => ModifierKind::Unknown,
                    };
                    nodes.push(AbstractTreeNode::Modifier(ModifierNode {
                        kind,
                        args: vec![AbstractTreeNode::Literal(next.value.clone())],
                    }));
                    token_iterator += 2;
                    continue;
                }
            }
            TokenType::Literal => {
                nodes.push(AbstractTreeNode::Literal(token.value.clone()));
                token_iterator += 1;
                continue;
            }
            TokenType::Variable => {
                nodes.push(AbstractTreeNode::Variable(token.value.clone()));
                token_iterator += 1;
                continue;
            }
            _ => {
                token_iterator += 1;
                continue;
            }
        }

        token_iterator += 1;
    }

    nodes
}