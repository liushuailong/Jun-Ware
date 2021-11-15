use style::{StyledNode, Display};
use css::Value::{Keyword, Length};
use css::Unit::Px;
use std::default::Default;

pub use self::BoxType::{AnonymousBlock, InlineNode, BlockNode};


#[derive(Clone, Copy, Defualt, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Defualt, Debug)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: Edgesizes,
    pub margin: Edgesizes
}


#[derive(Clone, Copy, Defualt, Debug)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
    BlockNode(&'a StyleNode<'a>),
    InlineNode(&'a StyleNode<'a>),
    AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn get_style_node(&self) -> &'a StyledNode<'a> {
        match self.box_type {
            BlockNode(node) | InlineNode(node) => node,
            AnonymousBlock => panic!("Anonmous block box has no style node")
        }
    }
}
// enum Display {
//     Inline,
//     Block,
//     None,
// }

// impl StyledNode {
//     fn value(&self, name: &str) -> Option<Value> {
//         self.specified_values.get(name).map(|v| v.clone())
//     }

//     fn display(&self) -> Display {
//         match self.value("display") {
//             Some(Keyword(s)) => match &*s {
//                 "block" => Dispaly::Block,
//                 "none" => Display::None,
//                 _ => Display::Inline
//             },
//             _ => Display::Inline
//         }
//     }
// }

fn layout_tree<'a>(node: &'a StyledNode<'a>, mut containing_block: Dimensions) -> LayoutBox<'a> {
    containing_block.content.height = 0.0;
    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

fn build_layout_tree<'a>(style_node: &'a StyledNode<'a>) -> Layoutbox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Block => BlockNode(style_node),
        Inline => InlineNode(style_node),
        DisplayNone => panic!("Root node has display: none.")
    });

    for child in &style_node.children {
        match child.display() {
            Block => root.children.push(build_layout_tree(child)),
            Inline => root.get_inline_container().children.push(build_layout_tree(child)),
            DisplayNone => {}
        }
    }

    return root;
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        layoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn layout(&mut self, mut containing_block: Dimensions) {
        match self.box_type {
            BlockNode(_) => self.layout_block(containing_block),
            InlineNode(_) | AnonymousBlock => {}
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions) {
        
    }
}

