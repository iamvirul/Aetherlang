use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct AetherString {
    value: String,
}

impl AetherString {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn concat(&self, other: &AetherString) -> AetherString {
        AetherString::new(format!("{}{}", self.value, other.value))
    }

    pub fn length(&self) -> usize {
        self.value.chars().count()
    }

    pub fn substring(&self, start: usize, end: Option<usize>) -> AetherString {
        let chars: Vec<char> = self.value.chars().collect();
        let end = end.unwrap_or(chars.len()).min(chars.len());
        
        if start >= chars.len() || start > end {
            AetherString::new("")
        } else {
            let result: String = chars[start..end].iter().collect();
            AetherString::new(result)
        }
    }

    pub fn to_uppercase(&self) -> AetherString {
        AetherString::new(self.value.to_uppercase())
    }

    pub fn to_lowercase(&self) -> AetherString {
        AetherString::new(self.value.to_lowercase())
    }

    pub fn trim(&self) -> AetherString {
        AetherString::new(self.value.trim())
    }
}

impl fmt::Display for AetherString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
} 