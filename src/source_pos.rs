/// Identifiers a location in a proto source file.
#[derive(Debug, Clone)]
pub struct SourcePos {
	filename: String,
	line: i32,
	col: i32,
	offset: i32
}

impl SourcePos {
	/// Returns a SourcePos with the given filename
	/// 
	/// # Arguments
	/// 
	/// * `name` - A string slice that holds the filename
	pub fn new(filename: &str) -> Self {
		return SourcePos{
			filename: filename.to_string(),
			line: 0,
			col: 0,
			offset: 0
		}
	}

	pub fn set_line(&mut self, line: i32) -> &mut Self {
		self.line = line;
		self
	}

	pub fn set_col(&mut self, col: i32) -> &mut Self {
		self.col = col;
		self
	}

	pub fn set_offset(&mut self, offset: i32) -> &mut Self {
		self.offset = offset;
		self
	}

	pub fn to_string(&self) -> String {
		if self.line <= 0 || self.col <= 0 {
			return self.filename.clone()
		}
		return format!("{}:{}:{}", self.filename, self.line, self.col)
	}
}

#[derive(Debug, Clone)]
pub struct PosRange {
	pub start: SourcePos,
	pub end: SourcePos
}

#[derive(Debug, Clone)]
pub struct Comment {
	pos_range: PosRange,
	leading_whitespace: String,
	text: String
}

impl Comment {
	pub fn new(pos_range: PosRange) -> Self {
		return Comment {
			pos_range,
			leading_whitespace: "".to_string(),
			text: "".to_string()
		}
	}

	pub fn set_leading_whitespace(&mut self, lw: &str) -> &mut Self {
		self.leading_whitespace = lw.to_string();
		self
	}

	pub fn set_text(&mut self, text: &str) -> &mut Self {
		self.text = text.to_string();
		self
	}
}