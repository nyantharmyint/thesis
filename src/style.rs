use dom::*;
use css::*;
use std::collections::HashMap;
#[derive(Debug)]
struct Style_List {
	Stylelist : Vec<Styled_Node>
}

#[derive(Debug)]
struct Styled_Node {
	node: Node,
	style : Vec<String>,
}



pub type Style_map = HashMap<String, Vec<Decleration>>;



pub fn style(mut html_nodes: Vec<Node>, s: StyleSheet) {

	let mut stymap = style_map(s);
	println!("{:#?}", stymap );
	refine_node(&mut html_nodes[0], &mut stymap);

	println!("The DOM : {:#?}", html_nodes );

}

fn style_map(s: StyleSheet) -> Style_map {
	let mut stymap = HashMap::new();
	let mut declerations = Vec::new();

	for rule in s.rules {
		declerations = rule.declerations;
		for selector in rule.selectors {

			stymap.insert(take_selector(selector), clone_dec(&declerations));
		}
	}
	stymap
	
}

fn refine_node(node:&mut Node, s : & Style_map) {

		let mut style: Vec<Decleration> = Vec::new();
		let V =vec![Decleration::new("".to_string(), Vec::new())];
		let tag_name = call_node_type(&node) ;
		let mut class = call_class(&node) ;
		let mut id = call_id(&node)  ;
		class = format!(".{}", class);
		id = format!("#{}",id);
		let mut wt =s.get(&tag_name);
		let mut wc =s.get(&class);
		let mut wid =s.get(&id);

		match wt {
			Some(t) => for g in t {
				style.push(copy_fk(g));
			},
			None => ()
		};

		match wc {
			Some(t) => for g in t {
				style.push(copy_fk(g));
			},
			None => ()
		};

		match wid {
			Some(t) => for g in t {
				style.push(copy_fk(g));
			},
			None => ()
		};


		node.add_style(style);

		for mut child in &mut node.children {
			refine_node(&mut child, s);
		}
	
}

fn copy_fk(gg: &Decleration) -> Decleration {
	let property = gg.property.clone();
	let mut value = Vec::new();

	for g in gg.value.iter() {
		value.push(g.clone());
	}

	Decleration {
		property, value
	}
}
