#![forbid(unsafe_code)]
#![deny(clippy::all)]

//! GNU C Library (glibc) compatibility shim for SigmaOS.

use std::fmt;

// ---------------------------------------------------------------------------
// GlibcVersion
// ---------------------------------------------------------------------------

/// Semantic version triple for a glibc release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlibcVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl GlibcVersion {
    /// Create a new version triple.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for GlibcVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "glibc-{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ---------------------------------------------------------------------------
// GlibcCompat
// ---------------------------------------------------------------------------

/// Represents the glibc compatibility context: version and symbol table size.
#[derive(Debug)]
pub struct GlibcCompat {
    pub version: GlibcVersion,
    pub symbol_count: usize,
}

impl GlibcCompat {
    /// Construct with the given version; sets `symbol_count` to the size of
    /// the built-in symbol table.
    pub fn new(version: GlibcVersion) -> Self {
        Self {
            version,
            symbol_count: GLIBC_SYMBOLS.len(),
        }
    }
}

// ---------------------------------------------------------------------------
// Symbol table
// ---------------------------------------------------------------------------

/// Hardcoded list of well-known glibc public symbols.
const GLIBC_SYMBOLS: [&str; 20] = [
    "malloc", "free", "calloc", "realloc", "printf", "fprintf", "sprintf", "snprintf", "fopen",
    "fclose", "fread", "fwrite", "memcpy", "memset", "memmove", "strlen", "strcpy", "strcat",
    "strcmp", "strdup",
];

/// Return `true` if `name` is a recognised glibc public symbol.
pub fn check_symbol(name: &str) -> bool {
    GLIBC_SYMBOLS.contains(&name)
}

/// Return the ABI version tag string used in ELF symbol versioning.
pub fn abi_tag() -> &'static str {
    "GLIBC_2.31"
}

/// Return the canonical on-disk path of the C library shared object.
pub fn libc_path() -> &'static str {
    "/lib/x86_64-linux-gnu/libc.so.6"
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- AAA Test 1: GlibcVersion Display format ---
    #[test]
    fn test_version_display() {
        // Arrange
        let version = GlibcVersion::new(2, 31, 0);

        // Act
        let display = version.to_string();

        // Assert
        assert_eq!(display, "glibc-2.31.0");
    }

    // --- AAA Test 2: check_symbol returns true for known symbol ---
    #[test]
    fn test_check_symbol_found() {
        // Arrange
        let known_symbols = ["malloc", "free", "printf", "memcpy", "strlen"];

        // Act + Assert
        for sym in &known_symbols {
            assert!(check_symbol(sym), "expected '{sym}' to be found");
        }
    }

    // --- AAA Test 3: check_symbol returns false for unknown symbol ---
    #[test]
    fn test_check_symbol_not_found() {
        // Arrange
        let unknown = "sigmaos_magic_fn";

        // Act
        let result = check_symbol(unknown);

        // Assert
        assert!(!result);
    }

    // --- AAA Test 4: abi_tag returns correct string ---
    #[test]
    fn test_abi_tag() {
        // Arrange / Act
        let tag = abi_tag();

        // Assert
        assert_eq!(tag, "GLIBC_2.31");
    }

    // --- AAA Test 5: libc_path returns correct path ---
    #[test]
    fn test_libc_path() {
        // Arrange / Act
        let path = libc_path();

        // Assert
        assert_eq!(path, "/lib/x86_64-linux-gnu/libc.so.6");
    }

    // --- AAA Test 6: GlibcCompat construction sets fields correctly ---
    #[test]
    fn test_glibccompat_construction() {
        // Arrange
        let version = GlibcVersion::new(2, 31, 0);

        // Act
        let compat = GlibcCompat::new(version);

        // Assert
        assert_eq!(compat.version, version);
        assert_eq!(compat.symbol_count, 20);
    }

    // --- AAA Test 7: full end-to-end shim check ---
    #[test]
    fn test_shim_end_to_end() {
        // Arrange
        let version = GlibcVersion::new(2, 35, 0);
        let compat = GlibcCompat::new(version);

        // Act
        let display = compat.version.to_string();
        let found = check_symbol("strdup");
        let missing = check_symbol("nonexistent");
        let tag = abi_tag();

        // Assert
        assert_eq!(display, "glibc-2.35.0");
        assert!(found);
        assert!(!missing);
        assert_eq!(tag, "GLIBC_2.31");
        assert_eq!(compat.symbol_count, 20);
    }
}
