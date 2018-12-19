use std::collections::HashMap;
use dom::*;
use parser::*;
use css::*;

impl Parser {

	fn consume_comment(&mut self) {

		loop {
			if self.consume_char() == '-' && 
			   self.get_char() == '-' && 
			   self.get_next_char() == '>'
			{
				self.consume_char();
				self.consume_char();
				break;
			}
		}
	}

	fn parse_tag(&mut self) -> String {

			self.consume_space();

			let mut tag = match self.consume_char() {
				'<' => self.consume_while(|c| c!=' ' && c!='>'),
				 _  => self.consume_while(|c| c!='<')
			};
			if tag == "br/" || tag == "hr/" {
				tag.pop();
			}
			tag
	}

	pub fn parse_html(&mut self) -> (Vec<Node>, StyleSheet){

		let mut element = String::new();
		let mut attributes = HashMap::new();
		let mut nodes = Vec::new();
		let mut css = StyleSheet::new();

		loop {

			if self.eof() {
				break;
			}

			self.consume_space();

			if self.get_char() == '<' {

				element = self.parse_tag();

				if element == "!--".to_string() {
					self.consume_comment();
					continue;
				}

				else {
					attributes = self.parse_attributes();
					if element == "style".to_string() {
						self.consume_char();
						let mut style = Parser::new(self.consume_while(|c| c!='<'));
						css = style.parse_css();
					}

					nodes.push(Node::new_node_Element(element.clone(), attributes));	
				}				
			}

			else {

				element =self.parse_tag() ;


				if element.starts_with('\n') || element == "" {
					self.consume_space();
					continue;				
				}

				nodes.push(Node::new_node_Text(element.clone() , Vec::new()));

				
			}
					
		}
		element.pop();

		(nodes, css)
	}
	
	fn parse_attributes(&mut self) -> Attr {

		let mut attr= HashMap::new();
		let mut value = String::new();

		loop {
			self.consume_space();
			if self.get_char() == '>' 
			 {
				break;
			}
			if self.get_char() == '/' && self.get_next_char() == '>' {
				self.consume_char();
				self.consume_space();
				break;
			}

			self.consume_space();
			let name = self.consume_while(|c| {c!=' ' && c!='='});
			self.consume_space();
			assert!(self.consume_char()== '=');
			self.consume_space();
			let quote = self.consume_char();

			if quote == '"' || quote =='\'' {
				value = self.consume_while(|c| {c != quote});
				attr.insert(name, value);
			}
			else {
				attr.insert(name, "".to_string());
			}
			assert!(self.consume_char() == quote);

			
		}
		attr
	}

}


