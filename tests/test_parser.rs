#[cfg(test)]
mod tests {
    //use super::*;
    use rust::ConfigParser;

    //testing section parsing
    #[test]
    fn test_parse_section() {
        let mut parser = ConfigParser::new();
        parser.parse("[section]").unwrap();
        assert!(parser.sections.contains_key("section"));
    }

    //testing key=val
    #[test]
    fn test_parse_key_value() {
        let mut parser = ConfigParser::new();
        parser.parse("[section]\nkey=value").unwrap();
        assert_eq!(parser.sections["section"]["key"], "value");
    }

    //testing comments parsing
    #[test]
    fn test_parse_comment() {
        let mut parser = ConfigParser::new();
        parser.parse("[section]\n# This is a comment\nkey=value").unwrap();
        assert_eq!(parser.sections["section"]["key"], "value");
    }

    //testing invalid line parsing
    #[test]
    fn test_invalid_line() {
        let mut parser = ConfigParser::new();
        let result = parser.parse("invalid_line");
        assert!(result.is_err());
    }
}
