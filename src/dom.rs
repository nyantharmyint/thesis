use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
	pub node_type: Node_Type,
	pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum Node_Type {
	Text(String),
	Element(Element_data),
}
#[derive(Debug)]
pub struct Element_data {
	pub name: String,
	pub attributes: Attr,

}

pub type Attr = HashMap<String, String>;
impl Node {
	
	pub fn new_node_Text( text: String, children: Vec<Node>) -> Node{
		
		Node {
			node_type : Node_Type::Text(text),
			children : children,
		}
		
	}
	

	pub fn new_node_Element(name : String, attributes : Attr) -> Node {

		Node {
			node_type : Node_Type::Element(
				Element_data{
					name,
					attributes
				}
			),
		children : Vec::new(),
	}
}
	pub fn push_child(&mut self, child: Node) {
		self.children.push(child);
		
	}

}

	pub fn call_node_type(g :& Node) -> String {
		match g.node_type {
			Node_Type::Element(ref element) => element.name.clone(),
			Node_Type::Text(ref _String) => "Text".to_string(),
		}
	}

	pub fn call_class(g :& Node) -> String {
		match g.node_type {
			Node_Type::Element(ref element) => match element.attributes.get("class") {
				Some(c) => c.clone(),
				None => " ".to_string(),
			},
			Node_Type::Text(ref _String) => " ".to_string(),
		}
	}

	pub fn call_id(g :& Node) -> String {
		match g.node_type {
			Node_Type::Element(ref element) => match element.attributes.get("id") {
				Some(c) => c.clone(),
				None => " ".to_string(),
			},
			Node_Type::Text(ref _String) => " ".to_string(),
		}
	}

	pub fn call_style(g :& Node) -> String {
		match g.node_type {
			Node_Type::Element(ref element) => match element.attributes.get("style") {
				Some(c) => c.clone(),
				None => " ".to_string(),
			},
			Node_Type::Text(ref _String) => " ".to_string(),
		}
	}

	pub fn dom(input : Vec<Node>) -> Vec<Node>{
	let mut temp : Vec<String> = Vec::new();
	let mut result : Vec<Node>  = Vec::new();
	temp.push("DOM".to_string());


	for node in input {

		let gg = call_node_type(&node);

		let mut last = match temp.last(){
			Some(i) => i.clone(),
			None => " ".to_string()
		};

		if gg.starts_with("/") {
			if gg == format!("/{}", last) {
				if gg!="/html" {
				let mut last_node  = result.pop().unwrap();
				
				let mut parent = match result.pop(){
					Some(Node) => Node,
					None => break
				};

				
					parent.push_child(last_node);
					result.push(parent);
					temp.pop();
				}
				else {
					break;
				}
				
			
			
			
		}
		continue;
	}

		else if gg == "Text".to_string() || gg == "link" || gg == "img" || gg == "br" 
		|| gg == "hr"{

			let mut last_node  = result.last_mut().unwrap();
			last_node.push_child(node);

			continue;
		}


		temp.push(gg);
		result.push(node);

	}
	
		result

}