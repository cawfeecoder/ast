use std::error::Error;

use crate::node::{TerminalNode, TokenInfo, CompositeNode, NodeTrait, RuneNode};
use crate::source_pos::{SourcePos, Comment};
use crate::identifiers::KeywordNode;
use std::fmt::Debug;

pub trait _ValueNodeTrait<T> {
    fn value(self) -> T;
}
pub trait ValueNodeTrait<T>: _ValueNodeTrait<T> + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_value_node_trait(&self) -> Box<dyn _ValueNodeTrait<T>>;
}

pub trait StringValueNode {
    fn as_string(self) -> String;
}

#[derive(Debug, Clone)]
pub struct StringLiteralNode {
    terminal_node: TerminalNode,
    val: String
}

impl StringLiteralNode {
    pub fn new(val: &str, info: TokenInfo) -> Self {
        return StringLiteralNode {
            terminal_node: info.as_terminal_node(),
            val: val.to_string()
        }
    }
}

impl StringValueNode for StringLiteralNode {
    fn as_string(self) -> String {
        return self.val.clone()
    }
}

impl _ValueNodeTrait<String> for StringLiteralNode {
    fn value(self) -> String {
        return self.as_string()
    }
}

impl NodeTrait for StringLiteralNode {
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

pub struct CompoundStringLiteralNode {
    composite_node: CompositeNode,
    val: String
}

impl CompoundStringLiteralNode {
    pub fn new(components: Vec<StringLiteralNode>) -> Result<Self, &'static str> {
        if components.len() == 0 {
            return Err("must have atlast one component")
        }
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(components.len());
        let mut b = "".to_string();
        for (i, c) in components.iter().enumerate() {
            children[i] = Box::new(c.clone());
            b.push_str(&c.clone().value());
        }
        let mut comp:CompositeNode = CompositeNode::new();
        comp.push_children(children);
        return Ok(CompoundStringLiteralNode {
            composite_node: comp,
            val: b
        })
    }
}

impl _ValueNodeTrait<String> for CompoundStringLiteralNode {
    fn value(self) -> String {
        return self.as_string()
    }
}

impl StringValueNode for CompoundStringLiteralNode {
    fn as_string(self) -> String {
        return self.val
    }
}

pub trait IntValueNodeTrait {
    fn as_int64(&self) -> (i64, bool);
    fn as_uint64(&self) -> (u64, bool);
}

#[derive(Clone)]
pub struct UintLiteralNode {
    terminal_node: TerminalNode,
    val: u64
}

impl UintLiteralNode {
    pub fn new(val: u64, info: TokenInfo) -> Self {
        return UintLiteralNode {
            terminal_node: info.as_terminal_node(),
            val: val
        }
    }
}

impl _FloatValueNodeTrait for UintLiteralNode {
    fn as_float(&self) -> f64 {
        return self.val as f64
    }
}

impl NodeTrait for UintLiteralNode {
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

impl _ValueNodeTrait<u64> for UintLiteralNode {
    fn value(self) -> u64 {
        return self.val
    }
}

impl IntValueNodeTrait for UintLiteralNode {
    fn as_int64(&self) -> (i64, bool) {
        if self.val as i64 > i64::MAX {
            return (0, false);
        }
        return (self.val as i64, true)
    }

    fn as_uint64(&self) -> (u64, bool) {
        return (self.val, true)
    }
}

pub struct PositiveUintLiteralNode {
    composite_node: CompositeNode,
    plus: RuneNode,
    uint: UintLiteralNode,
    val: u64
}

impl PositiveUintLiteralNode {
    pub fn new(sign: RuneNode, i: UintLiteralNode) -> PositiveUintLiteralNode {
        let children: Vec<Box<NodeTrait>> = vec![Box::new(sign.clone()), Box::new(i.clone())];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return PositiveUintLiteralNode {
            composite_node: comp,
            plus: sign,
            uint: i.clone(),
            val: i.val
        }
    }
}

impl _ValueNodeTrait<u64> for PositiveUintLiteralNode {
    fn value(self) -> u64 {
        return self.val
    }
}

impl IntValueNodeTrait for PositiveUintLiteralNode {
    fn as_int64(&self) -> (i64, bool) {
        if self.val as i64 > i64::MAX {
            return (0, false)
        }
        return (self.val as i64, true)
    }
    fn as_uint64(&self) -> (u64, bool) {
        return (self.val, true)
    }
}

struct NegativeIntLiteralNode {
    composite_node: CompositeNode,
    minus: RuneNode,
    uint: UintLiteralNode,
    val: i64
}

impl NegativeIntLiteralNode {
    fn new(sign: RuneNode, i: UintLiteralNode) -> Self {
        let children: Vec<Box<dyn NodeTrait>> = vec![Box::new(sign.clone()), Box::new(i.clone())];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return NegativeIntLiteralNode {
            composite_node: comp,
            minus: sign,
            uint: i.clone(),
            val: -1*(i.val as i64)
        }
    }
}

impl _ValueNodeTrait<i64> for NegativeIntLiteralNode {
    fn value(self) -> i64 {
        return self.val
    }
}

impl IntValueNodeTrait for NegativeIntLiteralNode {
    fn as_int64(&self) -> (i64, bool) {
        return (self.val, true)
    }

    fn as_uint64(&self) -> (u64, bool) {
        if self.val < 0 {
            return (0, false)
        }
        return (self.val as u64, true)
    }
}

pub trait _FloatValueNodeTrait {
    fn as_float(&self) -> f64;
}

impl Debug for _FloatValueNodeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("_FloatValueNodeTrait")
            .field("as_float", &self.as_float())
            .finish()
    }
}

pub trait FloatValueNodeTrait: _FloatValueNodeTrait + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_float_value_node_trait(&self) -> Box<dyn _FloatValueNodeTrait>;
}

impl Debug for dyn  FloatValueNodeTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("FloatValueNodeTrait")
            .field("as_node_trait", &self.as_node_trait())
            .field("as_float_value_node_trait", &self.as_float_value_node_trait())
            .finish()
    }
}

#[derive(Clone)]
pub struct FloatLiteralNode {
    terminal_node: TerminalNode,
    val: f64
}

impl FloatLiteralNode {
    pub fn new(val: f64, info: TokenInfo) -> Self {
        return FloatLiteralNode {
            terminal_node: info.as_terminal_node(),
            val: val
        }
    }
}

impl _FloatValueNodeTrait for FloatLiteralNode {
    fn as_float(&self) -> f64 {
        return self.val
    }
}

impl FloatValueNodeTrait for FloatLiteralNode {
    fn as_float_value_node_trait(&self) -> Box<dyn _FloatValueNodeTrait> {
        return Box::new(self.clone())
    }

    fn as_node_trait(&self) -> Box<dyn NodeTrait> {
        return Box::new(self.clone())
    }
}

impl _ValueNodeTrait<f64> for FloatLiteralNode {
    fn value(self) -> f64 {
        return self.val
    }
}

impl NodeTrait for FloatLiteralNode {
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

pub struct SpecialFloatLiteralNode {
    keyword_node: KeywordNode,
    val: f64
}

impl SpecialFloatLiteralNode {
    pub fn new(name: KeywordNode) -> Self {
        let f: f64;
        if name.clone().value() == "inf" {
            f = f64::INFINITY
        } else {
            f = f64::NAN
        }
        return SpecialFloatLiteralNode {
            keyword_node: name,
            val: f,
        }
    }
}

impl _FloatValueNodeTrait for SpecialFloatLiteralNode {
    fn as_float(&self) -> f64 {
        return self.val
    }
}

impl _ValueNodeTrait<f64> for SpecialFloatLiteralNode {
    fn value(self) -> f64 {
        return self.val
    }
}

#[derive(Debug)]
pub struct SignedFloatLiteralNode {
    composite_node: CompositeNode,
    sign: RuneNode,
    float: Box<dyn FloatValueNodeTrait>,
    val: f64
}

impl SignedFloatLiteralNode {
    pub fn new(sign: RuneNode, f: Box<dyn FloatValueNodeTrait>) -> Self {
        let children: Vec<Box<dyn NodeTrait>> = vec![Box::new(sign.clone()), f.as_node_trait()];
        let mut val = f.as_float_value_node_trait().as_float();
        let comp_chars: Vec<char> = "-".chars().collect();
        if sign.clone().rune() == comp_chars[0] {
            val = -1 as f64 * val;
        }
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return SignedFloatLiteralNode {
            composite_node: comp,
            sign,
            float: f,
            val
        }
    }
}

impl _ValueNodeTrait<f64> for SignedFloatLiteralNode {
    fn value(self) -> f64 {
        return self.val
    }
}

impl _FloatValueNodeTrait for SignedFloatLiteralNode {
    fn as_float(&self) -> f64 {
        return self.val
    }
}

struct BoolLiteralNode {
    keyword_node: KeywordNode,
    val: bool
}

impl BoolLiteralNode {
    pub fn new(name: KeywordNode) -> Self {
        return BoolLiteralNode {
            keyword_node: name.clone(),
            val: name.value() == "true"
        }
    }
}

impl _ValueNodeTrait<bool> for BoolLiteralNode {
    fn value(self) -> bool {
        return self.val
    }
}

struct ArrayLiteralNode<T> {
    composite_node: CompositeNode,
    open_bracket: RuneNode,
    elements: Vec<Box<dyn ValueNodeTrait<T>>>,
    commas: Vec<RuneNode>,
    close_bracket: RuneNode
}

impl<T> ArrayLiteralNode<T> {
    fn new(open_bracket: RuneNode, vals: Vec<Box<dyn ValueNodeTrait<T>>>, commas: Vec<RuneNode>, close_bracket: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity((vals.len()*2) + 1);
        children.push(Box::new(open_bracket.clone()));
        for (i, val) in vals.iter().enumerate() {
            if i > 0 {
                children.push(commas.get(i-1).unwrap().as_node_trait());
            }
            children.push(val.as_node_trait());
        }
        children.push(Box::new(close_bracket.clone()));

        let mut comp = CompositeNode::new();
        comp.push_children(children);

        return ArrayLiteralNode {
            composite_node: comp,
            open_bracket,
            elements: vals,
            commas: commas,
            close_bracket
        }
    }
}

impl<T> _ValueNodeTrait<Vec<Box<dyn ValueNodeTrait<T>>>> for ArrayLiteralNode<T> {
    fn value(self) -> Vec<Box<dyn ValueNodeTrait<T>>> {
        return self.elements
    }
}

struct MessageLiteralNode<T> {
    composite_node: CompositeNode,
    open: RuneNode,
    elements: Vec<MessageFieldNode<T>>,
    seps: Vec<RuneNode>,
    close: RuneNode
}

impl<T> MessageLiteralNode<T> { 
  fn new(open_sym: RuneNode, vals: Vec<MessageFieldNode<T>>, seps: Vec<RuneNode>, close_sym: RuneNode) -> Self {
    let mut num_children = vals.len() + 2;
    for (_, sep) in seps.iter().enumerate() {
        num_children += 1;
    }
    let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(num_children);
    children.push(Box::new(open_sym.clone()));
    for (i, val) in vals.iter().enumerate() {
        //children.push(val);
        children.push(seps.get(i).unwrap().as_node_trait());
    }
    children.push(Box::new(close_sym.clone()));

    let mut comp = CompositeNode::new();
    comp.push_children(children);

    return MessageLiteralNode {
        composite_node: comp,
        open: open_sym,
        elements: vals,
        seps: seps,
        close: close_sym
    }
  }
}

pub struct MessageFieldNode<T> {
    composite_node: CompositeNode,
    //name: FieldReferenceNode,
    sep: Option<RuneNode>,
    val: Box<dyn ValueNodeTrait<T>>
}

impl<T> MessageFieldNode<T> {
    fn new(sep: Option<RuneNode>, val: Box<dyn ValueNodeTrait<T>>) -> Self {
        let mut num_children = 2;
        if sep.is_some() {
            num_children += 1;
        }
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(num_children);
        //children.push(Box::new(name));
        if sep.is_some() {
            children.push(sep.clone().unwrap().as_node_trait());
        }
        children.push(val.as_node_trait());

        let mut comp = CompositeNode::new();
        comp.push_children(children);

        return MessageFieldNode {
            composite_node: comp,
            sep: sep,
            val: val
        }
    }
}

