use crate::node::{TerminalNode, TokenInfo, CompositeNode, RuneNode, NodeTrait};
use crate::values::{_ValueNodeTrait};
use crate::source_pos::{SourcePos, Comment};

pub type Identifier = String;

pub trait _IdentValueNodeTrait {
    fn as_identifier(&self) -> Identifier;
}

pub trait IdentValueNodeTrait: _IdentValueNodeTrait + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_ident_value_node_trait(&self) -> Box<dyn _IdentValueNodeTrait>;
}

#[derive(Clone)]
pub struct IdentNode {
    terminal_node: TerminalNode,
    val: String
}

impl IdentNode {
    pub fn new(val: &str, info: TokenInfo) -> Self {
        return IdentNode {
            terminal_node: info.as_terminal_node(),
            val: val.to_string()
        }
    }

    pub fn as_identifier(self) -> Identifier {
        return self.val
    }

    pub fn to_keyword(self) -> KeywordNode {
        return self
    }
}

impl NodeTrait for IdentNode {
    fn start(&self) -> &SourcePos {
        return &self.terminal_node.start()
    }

    fn end(&self) -> &SourcePos {
        return &self.terminal_node.end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.terminal_node.leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.terminal_node.trailing_comments()
    }
}

impl _ValueNodeTrait<Identifier> for IdentNode {
    fn value(self) -> Identifier {
        return self.as_identifier()
    }
}
#[derive(Clone)]
pub struct CompoundIdentNode {
    composite_node: CompositeNode,
    leading_dot: Option<RuneNode>,
    components: Vec<IdentNode>,
    dots: Vec<RuneNode>,
    val: String
}

impl CompoundIdentNode {
    fn new(leading_dot: Option<RuneNode>, components: Vec<IdentNode>, dots: Vec<RuneNode>) -> Self {
        let mut num_children = (components.len()*2) - 1;
        if leading_dot.is_some() {
            num_children += 1;
        }
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(num_children);
        let mut b = "".to_string();
        if leading_dot.is_some() {
            children.push(Box::new(leading_dot.clone().unwrap()));
            b.push(leading_dot.clone().unwrap().rune())
        }
        for (i, c) in components.iter().enumerate() {
            if i > 0 {
                let dot = dots.get(i-1);
                children.push(Box::new(dot.clone().unwrap().clone()));
                b.push(dot.clone().unwrap().rune());
            }
            children.push(Box::new(c.clone()));
            b.push_str(&c.clone().value());
        }
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return CompoundIdentNode {
            composite_node: comp,
            leading_dot: leading_dot,
            components: components,
            dots: dots,
            val: b
        }
    }
}

impl _ValueNodeTrait<String> for CompoundIdentNode {
    fn value(self) -> String {
        return self.val
    }
}

impl _IdentValueNodeTrait for CompoundIdentNode {
    fn as_identifier(&self) -> Identifier {
        return self.val.clone()
    }
}

pub type KeywordNode = IdentNode;