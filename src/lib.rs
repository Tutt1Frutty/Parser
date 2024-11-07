use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid line format")]
    InvalidLine,
    #[error("Missing section")]
    MissingSection,
}

#[derive(Debug, Default)]
pub struct ConfigParser {
    pub sections: HashMap<String, HashMap<String, String>>,
}

impl ConfigParser {
    pub fn new() -> Self {
        Self {
            sections: HashMap::new(),
        }
    }


    fn parse_line(&mut self, line: &str, current_section: &mut Option<String>) -> Result<(), ParseError> {
        let line = line.trim();

        // Ignore comments and empty lines
        if line.starts_with('#') || line.starts_with(';') || line.is_empty() {
            return Ok(());
        }

        // Check for section declaration
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line[1..line.len()-1].trim().to_string();
            self.sections.entry(section_name.clone()).or_default();
            *current_section = Some(section_name);
            return Ok(());
        }

        // Check for key-value pair
        if let Some((key, value)) = line.split_once('=') {
            let section = current_section.as_ref().ok_or(ParseError::MissingSection)?;
            let section_map = self.sections.get_mut(section).unwrap();
            section_map.insert(key.trim().to_string(), value.trim().to_string());
            return Ok(());
        }

        // If no valid rule is matched
        Err(ParseError::InvalidLine)
    }

    /// Parses the entire configuration content
    pub fn parse(&mut self, content: &str) -> Result<(), ParseError> {
        let mut current_section = None;
        for line in content.lines() {
            self.parse_line(line, &mut current_section)?;
        }
        Ok(())
    }
}
