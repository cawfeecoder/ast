use crate::node::{NodeTrait};
use crate::node::{CompositeNode, RuneNode};
use crate::identifiers::{KeywordNode, IdentNode};
use crate::values::{ValueNodeTrait, _ValueNodeTrait};
use crate::source_pos::{SourcePos, Comment};

pub trait OptionDeclNode<T> {
    fn get_name(&self) -> Box<dyn NodeTrait>;
    fn get_value(&self) -> Box<dyn _ValueNodeTrait<T>>;
}

#[derive(Clone)]
pub struct OptionNode<T> {
    composite_node: CompositeNode,
    keyword: Option<KeywordNode>,
    name: OptionNameNode,
    equals: RuneNode,
    val: Box<dyn ValueNodeTrait<T>>,
    semicolon: Option<RuneNode>
}

impl<T> OptionNode<T> {
    pub fn file_element() {}
    pub fn msg_element() {}
    pub fn one_of_element() {}
    pub fn enum_element() {}
    pub fn service_element() {}
    pub fn method_element() {}

    pub fn new(keyword: KeywordNode, name: OptionNameNode, equals: RuneNode, val: Box<dyn ValueNodeTrait<T>>, semicolon: RuneNode) -> Self {
        let children: Vec<Box<dyn NodeTrait>> = vec![Box::new(keyword.clone()), Box::new(equals.clone()), val.as_node_trait(), Box::new(semicolon.clone())];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return OptionNode {
            composite_node: comp,
            name: name,
            keyword: Some(keyword),
            equals: equals,
            val: val,
            semicolon: Some(semicolon)
        }
    }

    pub fn new_compact(name: OptionNameNode, equals: RuneNode, val: Box<dyn ValueNodeTrait<T>>) -> Self {
        let children: Vec<Box<dyn NodeTrait>> = vec![Box::new(equals.clone()), val.as_node_trait()];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return OptionNode {
            composite_node: comp,
            name: name,
            equals: equals,
            val: val,
            keyword: None,
            semicolon: None
        }
    }
}

impl<T: Clone> NodeTrait for OptionNode<T> {
    fn start(&self) -> &SourcePos {
        return &self.composite_node.start()
    }

    fn end(&self) -> &SourcePos {
        return &self.composite_node.end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.composite_node.leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.composite_node.trailing_comments()
    }
}

impl<T> OptionDeclNode<T> for OptionNode<T> {
    fn get_name(&self) -> Box<dyn NodeTrait> {
        return Box::new(self.name.clone())
    }

    fn get_value(&self) -> Box<dyn _ValueNodeTrait<T>> {
        return self.val.as_value_node_trait()
    }
}

#[derive(Clone)]
pub struct OptionNameNode {
    composite_node: CompositeNode,
    parts: Vec<FieldReferenceNode>,
    dots: Vec<RuneNode>
}

impl OptionNameNode {
    pub fn new(parts: Vec<FieldReferenceNode>, dots: Vec<RuneNode>) -> Self {
        let mut children: Vec<Box<NodeTrait>> = Vec::with_capacity((parts.len()*2)-1);
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                children.push(Box::new(dots.get(i-1).unwrap().clone()));
            }
            children.push(Box::new(part.clone()));
        }
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return OptionNameNode {
            composite_node: comp,
            parts,
            dots
        }
    }
}

impl NodeTrait for OptionNameNode {
    fn start(&self) -> &SourcePos {
        return &self.composite_node.start()
    }

    fn end(&self) -> &SourcePos {
        return &self.composite_node.end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.composite_node.leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.composite_node.trailing_comments()
    }
}

#[derive(Clone)]
pub struct FieldReferenceNode {
    composite_node: CompositeNode,
    open: Option<RuneNode>,
    name: IdentNode,
    close: Option<RuneNode>
}

impl FieldReferenceNode {
    pub fn new(name: IdentNode) -> Self {
        let mut children: Vec<Box<NodeTrait>> = vec![];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return FieldReferenceNode {
            composite_node: comp,
            name,
            open: None,
            close: None
        }
    }

    pub fn new_extrension(open_sym: RuneNode, name: IdentNode, close_sym: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = vec![Box::new(name.clone()), Box::new(open_sym.clone()), Box::new(close_sym.clone())];
        let mut comp = CompositeNode::new();
        comp.push_children(children);
        return FieldReferenceNode {
            composite_node: comp,
            open: Some(open_sym),
            name,
            close: Some(close_sym)
        }
    }

    pub fn is_extension(&self) -> bool {
        return self.open.is_some()
    }
}

impl NodeTrait for FieldReferenceNode {
    fn start(&self) -> &SourcePos {
        return &self.composite_node.start()
    }

    fn end(&self) -> &SourcePos {
        return &self.composite_node.end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.composite_node.leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.composite_node.trailing_comments()
    }
}

impl _ValueNodeTrait<String> for FieldReferenceNode {
    fn value(self) -> String {
        if self.open.is_some() {
            return self.open.unwrap().rune().to_string() + &self.name.as_identifier() + &self.close.unwrap().rune().to_string()
        } else {
            return self.name.as_identifier()
        }
    }
}

#[derive(Clone)]
pub struct CompactOptionsNode<T> {
    composite_node: CompositeNode,
    open_bracket: RuneNode,
    options: Vec<OptionNode<T>>,
    commas: Vec<RuneNode>,
    close_bracket: RuneNode
}

impl<T: Clone + 'static> CompactOptionsNode<T> {
    pub fn new(open_bracket: RuneNode, opts: Vec<OptionNode<T>>, commas: Vec<RuneNode>, close_bracket: RuneNode) -> Self {
        let mut children: Vec<Box<dyn NodeTrait>> = Vec::with_capacity((opts.len()*2)+1);
        children.push(Box::new(open_bracket.clone()));
        for (i, opt) in opts.iter().enumerate() {
            if i > 0 {
                children.push(Box::new(commas.get(i -1).unwrap().clone()));
            }
            children.push(Box::new(opt.clone()));
        }
        children.push(Box::new(close_bracket.clone()));
        let mut comp = CompositeNode::new();
        comp.push_children(children);

        return CompactOptionsNode {
            composite_node: comp,
            open_bracket,
            options: opts,
            commas,
            close_bracket
        }
    }

    pub fn get_elements(&self) -> Vec<OptionNode<T>> {
        return self.options.clone()
    }
}

impl<T: Clone> NodeTrait for CompactOptionsNode<T> {
    fn start(&self) -> &SourcePos {
        return &self.composite_node.start()
    }

    fn end(&self) -> &SourcePos {
        return &self.composite_node.end()
    }

    fn leading_comments(&self) -> Vec<Comment> {
        return self.composite_node.leading_comments()
    }

    fn trailing_comments(&self) -> Vec<Comment> {
        return self.composite_node.trailing_comments()
    }
}
