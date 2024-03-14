//! Provides many pre-compiled pomsky expressions for use in scanners.

mod gen;

/// Retrieve the compiled patterns for use with `regex` crate
pub fn patterns() -> &'static [(&'static str, &'static str)] {
    gen::PATTERNS
}

#[cfg(test)]
mod tests {
    #[test]
    fn non_empty() {
        assert_ne!(0, crate::patterns().len());
    }
}
