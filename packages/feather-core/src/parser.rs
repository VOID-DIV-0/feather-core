use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar/feather.pest"]
pub struct FeatherParser;

/// Abstract Syntax Tree (AST) representation for the Feather language.
#[derive(Debug, Clone)]
pub enum AbstractTreeNode {
    Command {
        module: ModuleNode,
        signatures: Vec<SignatureNode>,
    },
    Module(ModuleNode),
    Signature(SignatureNode),
    Data(DataNode),
    Literal(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub struct ModuleNode {
    pub name: String,
    pub args: Vec<AbstractTreeNode>,
}

#[derive(Debug, Clone)]
pub enum SignatureNode {
    On(String),      // platform
    Trace,
    Elapsed,
    Silent,
    Timeout(f64, String), // value, unit
}

#[derive(Debug, Clone)]
pub enum DataNode {
    Text(String),
    Record(String),
    Container(String),
}

pub fn parse(input: &str) -> Result<Vec<AbstractTreeNode>, Box<dyn std::error::Error>> {
    let parsed = FeatherParser::parse(Rule::command, input)?
        .next()
        .unwrap();
    
    let mut nodes = Vec::new();
    
    for pair in parsed.into_inner() {
        match pair.as_rule() {
            Rule::module => {
                nodes.push(parse_module(pair));
            }
            Rule::signatures => {
                let signatures = parse_signatures(pair);
                for sig in signatures {
                    nodes.push(AbstractTreeNode::Signature(sig));
                }
            }
            Rule::terminator => {
                // Skip terminators for now
            }
            _ => {}
        }
    }
    
    Ok(nodes)
}

fn parse_module(pair: Pair<Rule>) -> AbstractTreeNode {
    let mut module_name = String::new();
    let mut args = Vec::new();
    
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::module_name => {
                module_name = inner_pair.as_str().to_string();
            }
            Rule::module_args => {
                for arg_pair in inner_pair.into_inner() {
                    args.push(parse_data(arg_pair));
                }
            }
            _ => {}
        }
    }
    
    AbstractTreeNode::Module(ModuleNode {
        name: module_name,
        args,
    })
}

fn parse_signatures(pair: Pair<Rule>) -> Vec<SignatureNode> {
    let mut signatures = Vec::new();
    
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::signature_trace => {
                signatures.push(SignatureNode::Trace);
            }
            Rule::signature_elapsed => {
                signatures.push(SignatureNode::Elapsed);
            }
            Rule::signature_silent => {
                signatures.push(SignatureNode::Silent);
            }
            Rule::signature_timeout => {
                let mut number = 0.0;
                let mut unit = String::new();
                
                for timeout_pair in inner_pair.into_inner() {
                    match timeout_pair.as_rule() {
                        Rule::number => {
                            number = timeout_pair.as_str().parse().unwrap_or(0.0);
                        }
                        Rule::unit_time => {
                            unit = timeout_pair.as_str().to_string();
                        }
                        _ => {}
                    }
                }
                
                signatures.push(SignatureNode::Timeout(number, unit));
            }
            Rule::signature_on => {
                for platform_pair in inner_pair.into_inner() {
                    signatures.push(SignatureNode::On(platform_pair.as_str().to_string()));
                }
            }
            _ => {}
        }
    }
    
    signatures
}

fn parse_data(pair: Pair<Rule>) -> AbstractTreeNode {
    match pair.as_rule() {
        Rule::data_text => {
            let text = pair.into_inner().as_str().to_string();
            AbstractTreeNode::Data(DataNode::Text(text))
        }
        Rule::data_record => {
            let record = pair.into_inner().next().unwrap().as_str().to_string();
            AbstractTreeNode::Data(DataNode::Record(record))
        }
        Rule::data_container => {
            let container = pair.into_inner().next().unwrap().as_str().to_string();
            AbstractTreeNode::Data(DataNode::Container(container))
        }
        _ => {
            AbstractTreeNode::Literal(pair.as_str().to_string())
        }
    }
}

// Keep the old parser function for backward compatibility if needed
pub fn parser(input: &str) -> Vec<AbstractTreeNode> {
    match parse(input) {
        Ok(nodes) => nodes,
        Err(_) => Vec::new(),
    }
}