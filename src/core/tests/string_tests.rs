#[cfg(test)]
mod tests {
    use crate::core::string::AetherString;

    #[test]
    fn test_string_creation() {
        let s = AetherString::new("Hello");
        assert_eq!(s.to_string(), "Hello");
        
        // Test empty string
        let empty = AetherString::new("");
        assert_eq!(empty.to_string(), "");
        
        // Test string with special characters
        let special = AetherString::new("Hello\n\t世界");
        assert_eq!(special.to_string(), "Hello\n\t世界");
    }

    #[test]
    fn test_string_concat() {
        let s1 = AetherString::new("Hello, ");
        let s2 = AetherString::new("World!");
        let result = s1.concat(&s2);
        assert_eq!(result.to_string(), "Hello, World!");

        // Test concatenation with empty strings
        let empty = AetherString::new("");
        assert_eq!(s1.concat(&empty).to_string(), "Hello, ");
        assert_eq!(empty.concat(&s1).to_string(), "Hello, ");
        assert_eq!(empty.concat(&empty).to_string(), "");
    }

    #[test]
    fn test_string_length() {
        let s = AetherString::new("Hello");
        assert_eq!(s.length(), 5);

        // Test empty string length
        let empty = AetherString::new("");
        assert_eq!(empty.length(), 0);

        // Test unicode string length
        let unicode = AetherString::new("Hello世界");
        assert_eq!(unicode.length(), 7);
    }

    #[test]
    fn test_substring() {
        let s = AetherString::new("Hello, World!");
        assert_eq!(s.substring(0, Some(5)).to_string(), "Hello");
        assert_eq!(s.substring(7, None).to_string(), "World!");
        
        // Edge cases
        assert_eq!(s.substring(100, None).to_string(), "");
        assert_eq!(s.substring(0, Some(100)).to_string(), "Hello, World!");
        assert_eq!(s.substring(5, Some(2)).to_string(), "");
        assert_eq!(s.substring(0, Some(0)).to_string(), "");
        
        // Unicode handling
        let unicode = AetherString::new("Hello世界");
        assert_eq!(unicode.substring(5, Some(7)).to_string(), "世界");
    }

    #[test]
    fn test_case_conversion() {
        let s = AetherString::new("Hello");
        assert_eq!(s.to_uppercase().to_string(), "HELLO");
        assert_eq!(s.to_lowercase().to_string(), "hello");

        // Test mixed case
        let mixed = AetherString::new("HeLLo WoRLD");
        assert_eq!(mixed.to_uppercase().to_string(), "HELLO WORLD");
        assert_eq!(mixed.to_lowercase().to_string(), "hello world");

        // Test with numbers and special characters
        let special = AetherString::new("Hello123!@#");
        assert_eq!(special.to_uppercase().to_string(), "HELLO123!@#");
        assert_eq!(special.to_lowercase().to_string(), "hello123!@#");
    }

    #[test]
    fn test_trim() {
        let s = AetherString::new("  Hello  ");
        assert_eq!(s.trim().to_string(), "Hello");

        // Test with different whitespace characters
        let whitespace = AetherString::new("\n\t Hello \n\t ");
        assert_eq!(whitespace.trim().to_string(), "Hello");

        // Test with no whitespace
        let no_whitespace = AetherString::new("Hello");
        assert_eq!(no_whitespace.trim().to_string(), "Hello");

        // Test empty string
        let empty = AetherString::new("");
        assert_eq!(empty.trim().to_string(), "");

        // Test only whitespace
        let only_whitespace = AetherString::new("   \n\t   ");
        assert_eq!(only_whitespace.trim().to_string(), "");
    }
} 