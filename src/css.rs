use dom::*;
use parser::*;

#[derive(Debug)]
pub struct StyleSheet {
 rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
	pub selectors: Vec<Selector>,
	pub declerations: Vec<Decleration>,
}
#[derive(Debug)]
pub enum Selector {
	Tag_Name(String),
	Id(String),
	Class(String),
}

#[derive(Debug)]
pub struct Decleration {
	property: String,
	value: Vec<String>,
}

impl Decleration {
	fn new(property: String, value: Vec<String>) -> Decleration {
		Decleration {
			property,
			value,
		}
	}
}
impl Rule {
	fn new(selectors: Vec<Selector>, declerations: Vec<Decleration>) -> Rule {
		Rule {
			selectors,
			declerations,
		}
	}
}

impl StyleSheet {
	pub fn new() -> StyleSheet {
		StyleSheet {
			rules: Vec::new(),
		}
	}
}

impl Parser {


	fn parse_selectors(&mut self) -> Vec<Selector>{

		let mut result = Vec::new();

		let mut selector;
		loop {
			self.consume_space();

			let current_char = self.get_char();
			if current_char == '{'|| self.eof() {

				break;
			}

			else if current_char == ',' || current_char == ' '{
				self.consume_char();
				continue;
			}

			let item = self.consume_while(|c| c!=' ' && c!= '{');

			if item.starts_with('.') {
				selector = Selector::Class(item);
				}	

			else if item.starts_with('#') {
				selector = Selector::Id(item) ;
			}
			else {
				selector = Selector::Tag_Name(item);
			}
			result.push(selector);
		}
		result
	}

	fn parse_declerations(&mut self) -> Vec<Decleration> {
		let mut result = Vec::new();

		loop {

			let current_char = self.get_char();
			if current_char == '}' || self.eof() {
				break;
			}
			else if current_char == '\n' {
				self.consume_char();
				self.consume_space();
				continue;
			}

			self.consume_space();
			let property = self.parse_property();
			if property == '\n'.to_string() {
				continue;
			}

			self.consume_space();
			assert!(self.consume_char()== ':');

			self.consume_space();
			let values = self.parse_values();
			self.consume_space();
			assert!(self.consume_char() == ';');

			let decleration = Decleration::new(property, values);
			result.push(decleration);
		}
		result


	}

	fn parse_property(&mut self) -> String {

		self.consume_while(|c| c!=':' && c!=' ')
	}

	fn parse_values(&mut self) -> Vec<String> {
		let mut result = Vec::new();

		loop {
			if self.get_char() == ';' {
				break;
			}
			self.consume_space();
			let value = self.consume_while(|c| c!=' ' && c!=';');
			
			result.push(value);
			self.consume_space();

		}
		result
	}



	pub fn parse_css(&mut self) -> StyleSheet {
		let mut stylesheet = StyleSheet::new() ;
		loop {

			self.consume_space();
			if self.eof() {
				break;
			}
			if self.get_char() == '\n' || self.get_char() == '\t' {
				self.consume_space();
				continue;
			}
			self.consume_space();
			let  selectors = self.parse_selectors();
			self.consume_space();
			self.consume_char();

			self.consume_space();
			let declerations = self.parse_declerations();

			assert!(self.consume_char() == '}');

			let rule = Rule::new(selectors, declerations);
			stylesheet.rules.push(rule);
		}

		 stylesheet

	}
}

 pub fn merge_css(first: StyleSheet, second: StyleSheet) -> StyleSheet {
	let mut rules = Vec::new();
	for r in first.rules{
		rules.push(r);
	}
	for r in second.rules{
		rules.push(r);
	}

	StyleSheet {
		rules,
	}

}
