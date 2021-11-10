use style::{StyledNode, Display}

struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: Edgesizes,
    margin: Edgesizes
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
}

enum BoxType<'a> {
    BlockNode(&'a StyleNode<'a>),
    InlineNode(&'a StyleNode<'a>),
    AnonymousBlock,
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

impl LayoutBox {
    fn new(box_type: BoxType) -> LayoutBox {
        layoutBox {
            box_type: box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }
}

