use crate::node::{NodeTrait, CompositeNode, RuneNode};
use crate::identifiers::{KeywordNode, IdentNode, IdentValueNodeTrait};

pub trait MessageDeclNodeTrait {
    fn message_name(&self) -> Box<dyn NodeTrait>;
}

pub struct MessageNode {
    composite_node: CompositeNode,
    keyword: KeywordNode,
    name: IdentNode,
    message_body: MessageBody
}

impl MessageNode {
    pub fn file_element(&self) {}
    pub fn msg_element(&self) {}
    pub fn new(keyword: KeywordNode, name: IdentNode, open_brace: RuneNode, decls: Vec<Box<dyn MessageElementTrait>>, close_brace: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(4 + decls.len());
        children.push(Box::new(keyword.clone()));
        children.push(Box::new(name.clone()));
        children.push(Box::new(open_brace.clone()));
        for (_, decl) in decls.iter().enumerate() {
            children.push(decl.as_node_trait());
        }
        children.push(Box::new(close_brace.clone()));
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        let mut msg_body = MessageBody::new();
        msg_body.populate_body(open_brace.clone(), decls, close_brace.clone());
        return MessageNode {
            composite_node: comp,
            keyword,
            name,
            message_body: msg_body
        }
    }
}

impl MessageDeclNodeTrait for MessageNode {
    fn message_name(&self) -> Box<dyn NodeTrait> {
        return Box::new(self.name.clone());
    }
}

pub struct MessageBody {
    open_brace: Option<RuneNode>,
    decls: Vec<Box<dyn MessageElementTrait>>,
    close_brace: Option<RuneNode>
}

impl MessageBody {
    pub fn new() -> Self {
        return MessageBody {
            open_brace: None,
            decls: vec![],
            close_brace: None
        }
    }

    pub fn populate_body(&mut self, open_brace: RuneNode, decls: Vec<Box<dyn MessageElementTrait>>, close_brace: RuneNode) -> &mut Self {
        self.open_brace = Some(open_brace);
        self.close_brace = Some(close_brace);
        self.decls = decls;
        return self
    }
}

pub trait _MessageElementTrait {
    fn msg_element(&self) {} 
}

pub trait MessageElementTrait: _MessageElementTrait + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_message_element_trait(&self) -> Box<dyn _MessageElementTrait>;
}

pub struct ExtendNode {
    composite_node: CompositeNode,
    keyword: KeywordNode,
    extendee: Box<dyn IdentValueNodeTrait>,
    open_brace: RuneNode,
    decls: Vec<Box<dyn ExtendElementTrait>>,
    close_brace: RuneNode
}

impl ExtendNode {
    pub fn file_element(&self) {}
    pub fn msg_element(&self) {}
    pub fn new(keyword: KeywordNode, extendee: Box<dyn IdentValueNodeTrait>, open_brace: RuneNode, decls: Vec<Box<dyn ExtendElementTrait>>, close_brace: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity(4 + decls.len());
        children.push(Box::new(keyword));
        children.push(extendee.as_node_trait());
        children.push(Box::new(open_brace));
        for (_, decl) in decls.iter().enumerate() {
            children.push(decl.as_node_trait());
        }
        children.push(Box::new(close_brace));

        let mut comp = CompositeNode::new();
        comp.push_children(children);

        let ret = ExtendNode {
            composite_node: comp,
            keyword,
            extendee,
            open_brace,
            decls,
            close_brace
        };
        for (_, decl) in decls.iter().enumerate() {
            match *decl {
                FieldNode => {
                    decl.extendee = ret
                },
                GroupNode => {
                    decl.extendee = ret
                }
                default => {
                    continue
                }
            }
        }
        return ret
    }
}

pub trait _ExtendElementTrait {
    fn extend_element(&self) {}
}

pub trait ExtendElementTrait: _ExtendElementTrait + NodeTrait {
    fn as_node_trait(&self) -> Box<dyn NodeTrait>;
    fn as_extend_element_trait(&self) -> Box<dyn _ExtendElementTrait>;
}
