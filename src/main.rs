use std::env;
use std::fs;

pub mod html;
pub mod dom;
pub mod parser;
pub mod css;
pub mod style;
fn main() {
	//taking input string (later change to file)
	let (html,css)  =  take_input(env::args());
	let html_file = fs::read_to_string(html).unwrap();
	println!("Input HTML : \n{}", html_file );

	let css_file = fs::read_to_string(css).unwrap();
	println!("\nInput CSS : \n{}", css_file);

	let mut to_parse = parser::Parser::new(html_file);

	let (html_nodes, internal_css) = to_parse.parse_html();
	let dom = dom::dom(html_nodes);
	println!("The Dom : \n {:#?}",dom );
	let mut to_parse_css = parser::Parser::new(css_file);
	let external_css = to_parse_css.parse_css();
	let merged_css = css::merge_css(external_css, internal_css);
	println!("{:#?}", merged_css );

}

fn take_input(mut args: std::env::Args) -> (String, String) {
	args.next();
	let html = match args.next() {
		Some(i) => i,
		None => panic!("There is no input"),
	};

	let css = match args.next() {
		Some(i) => i,
		None => panic!("There is no input for css file"),
	};
	(html, css)
}