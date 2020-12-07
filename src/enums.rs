use crate::node::{CompositeNode, RuneNode, NodeTrait};
use crate::identifiers::{IdentNode, KeywordNode};
use crate::values::{IntValueNodeTrait};
use crate::options::{CompactOptionsNode};

pub struct EnumNode {
    composite_node: CompositeNode,
    keyword: KeywordNode,
    name: IdentNode,
    open_brace: RuneNode,
    decls: Vec<Box<dyn EnumElementTrait>>,
    close_brace: RuneNode
}

impl EnumNode {
    pub fn file_element() {}
    pub fn msg_element() {}
    pub fn new(keyword: KeywordNode, name: IdentNode, open_brace: RuneNode, decls: Vec<Box<dyn EnumElementTrait>>, close_brace: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(4 + decls.len());
        children.push(Box::new(keyword.clone()));
        children.push(Box::new(name.clone()));
        children.push(Box::new(open_brace.clone()));
        for (_, decl) in decls.iter().enumerate() {
            children.push(decl.as_node_trait().clone());
        }
        children.push(Box::new(close_brace.clone()));

        let mut comp = CompositeNode::new();
        comp.push_children(children);
        
        return EnumNode {
            composite_node: comp,
            keyword,
            name,
            open_brace,
            close_brace,
            decls
        }
    }
}

pub trait _EnumElementTrait {
    fn enum_element(&self);
}

pub trait EnumElementTrait: _EnumElementTrait + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_enum_element_trait(&self) -> Box<dyn _EnumElementTrait>;
}

pub trait EnumValueDeclNodeTrait {
    fn get_name(&self) -> Box<dyn NodeTrait>;
    fn get_number(&self) -> Box<dyn NodeTrait>;
}

pub struct EnumValueNode<T> {
    composite_node: CompositeNode,
    name: IdentNode,
    equals: RuneNode,
    number: Box<dyn IntValueNodeTrait>,
    options: CompactOptionsNode<T>,
    semicolon: RuneNode
}

impl<T> _EnumElementTrait for EnumValueNode<T> {
    fn enum_element(&self) {} 
}

impl<T: Clone + 'static> EnumValueNode<T> {
    pub fn new(name: IdentNode, equals: RuneNode, number: Box<dyn IntValueNodeTrait>, opts: CompactOptionsNode<T>, semicolon: RuneNode) -> Self {
        let mut num_children = 4;
        if opts.get_elements().len() > 0 {
            num_children += 1;
        }
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(num_children);
        children.push(Box::new(name.clone()));
        children.push(Box::new(equals.clone()));
        children.push(number.as_node_trait());
        if opts.get_elements().len() > 0 {
            children.push(Box::new(opts.clone()));
        }
        children.push(Box::new(semicolon.clone()));
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return EnumValueNode {
            composite_node: comp,
            name,
            equals,
            number,
            options: opts,
            semicolon
        }
    }
}

impl <T> EnumValueDeclNodeTrait for EnumValueNode<T> {
    fn get_name(&self) -> Box<dyn NodeTrait> {
        return Box::new(self.name.clone())
    }

    fn get_number(&self) -> Box<dyn NodeTrait> {
        return self.number.as_node_trait()
    }
}
