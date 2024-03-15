//! Provides many pre-compiled pomsky expressions for use in scanners.

#![no_std]

include!(concat!(env!("OUT_DIR"), "/patterns.rs"));

/// Retrieve the compiled patterns for use with `regex` crate
pub fn patterns() -> &'static [(&'static str, &'static str)] {
    PATTERNS
}

#[cfg(test)]
mod tests {
    #[test]
    fn non_empty() {
        assert_ne!(0, crate::patterns().len());
    }
}
