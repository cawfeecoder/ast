use crate::source_pos::SourcePos;
use crate::source_pos::Comment;
use crate::source_pos::PosRange;
use crate::values::ValueNodeTrait;
use crate::values::_ValueNodeTrait;
#[macro_use]
use dyn_clone::{self, clone_trait_object, DynClone};

pub trait NodeTrait: DynClone {
    fn start(&self) -> &SourcePos;
    fn end(&self) -> &SourcePos;
    fn leading_comments(&self) -> Vec<Comment>;
    fn trailing_comments(&self) -> Vec<Comment>;
}

clone_trait_object!(NodeTrait);

impl std::fmt::Debug for dyn NodeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("NodeTrait")
            .field("start", &self.start())
            .field("end", &self.end())
            .field("leading_comments", &self.leading_comments())
            .field("trailing_comments", &self.trailing_comments())
            .finish()
    }
}

pub trait TerminalNodeTrait: NodeTrait {
    fn pop_leading_comment(&mut self) -> Option<Comment>;
    fn push_trailing_comment(&mut self, c: Comment);
    fn leading_whitespace(&self) -> &str;
    fn raw_text(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct TerminalNode {
    pos_range: PosRange,
    leading_comments: Vec<Comment>,
    leading_whitespace: String,
    trailing_comments: Vec<Comment>,
    raw: String
}

impl TerminalNode {
    pub fn new(pos_range: PosRange) -> Self {
        return TerminalNode {
            leading_comments: vec![],
            trailing_comments: vec![],
            leading_whitespace: "".to_string(),
            raw: "".to_string(),
            pos_range: pos_range
        }
    }

    pub fn set_leading_comments(&mut self, c: Vec<Comment>) -> &mut Self {
        self.leading_comments = c;
        self
    }

    pub fn set_trailing_comments(&mut self, c: Vec<Comment>) -> &mut Self {
        self.trailing_comments = c;
        self
    }

    pub fn set_leading_whitespace(&mut self, lw: &str) -> &mut Self {
        self.leading_whitespace = lw.to_string();
        self
    }

    pub fn set_raw(&mut self, raw: &str) -> &mut Self {
        self.raw = raw.to_string();
        self
    }
}

impl TerminalNodeTrait for TerminalNode {
    fn pop_leading_comment(&mut self) -> Option<Comment> {
        return self.leading_comments.pop()
    }

    fn push_trailing_comment(&mut self, c: Comment) {
        self.trailing_comments.push(c);
    }

    fn leading_whitespace(&self) -> &str {
        &self.leading_whitespace
    }

    fn raw_text(&self) -> &str {
        &self.raw
    }
}

impl NodeTrait for TerminalNode {
    fn start(&self) -> &SourcePos {
        return &self.pos_range.start
    }

    fn end(&self) -> &SourcePos {
        return &self.pos_range.end
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.leading_comments.clone()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.trailing_comments.clone()
    }
}

#[derive(Clone)]
pub struct TokenInfo {
    pos_range: PosRange,
    raw_text: String,
    leading_comments: Vec<Comment>,
    leading_whitespace: String,
    trailing_comments: Vec<Comment>
}

impl TokenInfo {
    pub fn new(pos_range: PosRange) -> Self {
        return TokenInfo {
            pos_range,
            raw_text: "".to_string(),
            leading_comments: vec![],
            leading_whitespace: "".to_string(),
            trailing_comments: vec![]
        }
    }

    pub fn set_raw_text(&mut self, raw: &str) -> &mut Self {
        self.raw_text = raw.to_string();
        self
    }

    pub fn set_leading_comments(&mut self, c: Vec<Comment>) -> &mut Self {
        self.leading_comments = c;
        self
    }

    pub fn set_leading_whitespace(&mut self, lw: &str) -> &mut Self {
        self.leading_whitespace = lw.to_string();
        self
    }

    pub fn set_trailing_comments(&mut self, c: Vec<Comment>) -> &mut Self {
        self.trailing_comments = c;
        self
    }

    pub fn as_terminal_node(&self) -> TerminalNode {
        return TerminalNode {
            pos_range: self.pos_range.clone(),
            leading_comments: self.leading_comments.clone(),
            leading_whitespace: self.leading_whitespace.clone(),
            trailing_comments: self.trailing_comments.clone(),
            raw: self.raw_text.clone()
        }
    }
}

pub trait CompositeNodeTrait: NodeTrait {
    fn children(&self) -> Vec<Box<dyn NodeTrait>>;
}

#[derive(Debug, Clone)]
pub struct CompositeNode {
    children: Vec<Box<dyn NodeTrait>>
}

impl CompositeNodeTrait for CompositeNode {
    fn children(&self) -> Vec<Box<dyn NodeTrait>> {
        self.children.clone()
    }
}

impl NodeTrait for CompositeNode {
    fn start(&self) -> &SourcePos {
        return &self.children[0].start()
    }

    fn end(&self) -> &SourcePos {
        return &self.children[self.children.len() - 1].end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.children[0].leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.children[self.children.len() - 1].trailing_comments()
    }
}

impl CompositeNode {
    pub fn new() -> Self {
        return CompositeNode {
            children: vec![]
        }
    }

    pub fn push_child(&mut self, c: Box<dyn NodeTrait>) {
        self.children.push(c);
    } 

    pub fn push_children(&mut self, mut c: Vec<Box<dyn NodeTrait>>) {
        self.children.append(&mut c);
    }
}

#[derive(Debug, Clone)]
pub struct RuneNode {
    terminal_node: TerminalNode,
    rune: char
}

impl RuneNode {
    pub fn new(c: char, info: TokenInfo) -> Self {
        return RuneNode {
            terminal_node: info.as_terminal_node(),
            rune: c
        }
    }

    pub fn rune(&self) -> char {
        self.rune
    }
}

impl ValueNodeTrait<char> for RuneNode {
    fn as_node_trait(&self) -> Box<dyn NodeTrait> {
        return Box::new(self.clone())
    }

    fn as_value_node_trait(&self) -> Box<dyn _ValueNodeTrait<char>> {
        return Box::new(self.clone())
    }
}

impl _ValueNodeTrait<char> for RuneNode {
    fn value(self) -> char {
        return self.rune
    }
}

impl NodeTrait for RuneNode {
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

pub struct EmptyDeclNode {
    composite_node: CompositeNode,
    semicolon: RuneNode
}

impl EmptyDeclNode {
    pub fn new(semicolon: RuneNode) -> Self {
        let c = CompositeNode {
            children: vec![Box::new(semicolon.clone())]
        };
        return EmptyDeclNode {
            composite_node: c,
            semicolon: semicolon
        }
    }

    pub fn file_element() {}
    pub fn msg_element() {}
    pub fn extend_element() {}
    pub fn one_of_element() {}
    pub fn enum_element() {}
    pub fn service_element() {}
    pub fn method_element() {}
}