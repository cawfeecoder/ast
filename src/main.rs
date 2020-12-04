mod node;
use node::{CompositeNode, TerminalNode, TokenInfo};
use node::NodeTrait;
use node::TerminalNodeTrait;
use node::RuneNode;
mod source_pos;
use source_pos::PosRange;
use source_pos::Comment;
use source_pos::SourcePos;
use std::char;
mod values;
use values::{SignedFloatLiteralNode, FloatLiteralNode};
mod identifiers;
use identifiers::{IdentNode};

fn main() {
    let s = SourcePos::new("test.yaml");
    let mut e = s.clone();
    e.set_line(5).set_col(10).set_offset(10);
    let p = PosRange {
        start: s.clone(),
        end: e.clone()
    };
    let mut t = TerminalNode::new(p.clone());
    let mut c = Comment::new(p.clone());
    t.push_trailing_comment(c);
    let mut ti = TokenInfo::new(p.clone());
    let mut r = RuneNode::new(char::from_u32(0x2764).unwrap(), ti.clone());
    let children: Vec<Box<dyn NodeTrait>> = vec![Box::new(r.clone()), Box::new(t)];
    let mut cn = CompositeNode::new();
    cn.push_children(children);
    let fln = FloatLiteralNode::new(1 as f64, ti.clone());
    let float_n = SignedFloatLiteralNode::new(r, Box::new(fln));
    println!("{:?}", float_n);
}