pub struct Parser {
	pub position: usize,
	pub input: String
}
impl Parser {
	pub fn new(input : String) -> Parser {
		Parser {
			position: 0,
			input: input,
		}
	}
	//end of file
	//compare current position with input length
	pub fn eof(&self) -> bool {
	 self.position >= self.input.len()
	}


	//move to next char
	pub fn next_char(&self) -> char {
		self.input[self.position..].chars().next().unwrap()
	}

	//return current char and move to next position.
	pub fn consume_char(&mut self) -> char {
		let mut iter = self.input[self.position..].char_indices();
		let (_, current_char) = iter.next().unwrap();
		let (next_jump, _) = iter.next().unwrap_or((1,' '));
		self.position += next_jump;
		current_char
	}

	pub fn get_char(&self) -> char {
		let (_, result) = self.input[self.position..].char_indices().next().unwrap_or((1,' '));
		result
	}

	pub fn get_next_char(&self) -> char {
		let next_pos = self.position +1;
		let (_, result) = self.input[next_pos..].char_indices().next().unwrap_or((1,' '));
		result
	}


	//while not end of file and given function is satisfied, 
	//consume current char.
	pub fn consume_while<F>(&mut self, test: F) -> String
		where F: Fn(char) -> bool {
			let mut result = String::new();
			while !self.eof() && test(self.next_char()) {
				result.push(self.consume_char());
			}
			result
		}


	//skip whitespaces.
	pub fn consume_space(&mut self) {
		self.consume_while(char::is_whitespace);
	}




}