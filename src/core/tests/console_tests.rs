#[cfg(test)]
mod tests {
    use crate::core::console::{Console, Color};

    #[test]
    fn test_print() {
        // These should not panic
        Console::print("Hello");
        Console::println("World");
    }

    #[test]
    fn test_print_colored() {
        // Test all colors
        Console::print_colored("Red text", Color::Red);
        Console::print_colored("Green text", Color::Green);
        Console::print_colored("Blue text", Color::Blue);
        Console::print_colored("Yellow text", Color::Yellow);
    }

    #[test]
    fn test_format() {
        let formatted = Console::format("Test");
        assert_eq!(formatted, "Test");
        
        let number_formatted = Console::format(42);
        assert_eq!(number_formatted, "42");
    }

    #[test]
    fn test_clear() {
        // Should not panic
        Console::clear();
    }
} 