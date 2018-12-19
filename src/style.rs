use dom::*;
use css::*;
#[derive(Debug)]
struct Style_List {
	Stylelist : Vec<Styled_Node>
}

#[derive(Debug)]
struct Styled_Node {
	node: Node,
	style : Vec<String>,
}


fn style(html_nodes: Vec<Node>, css_rules: Vec<StyleSheet>) {

	for node in html_nodes {
		let id = call_id(&node);
		let class = call_class(&node);
		let tag_name = call_node_type(&node);
	}

  
}


