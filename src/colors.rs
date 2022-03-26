pub mod colors {
    pub const _GREEN: &str = "\x1b[32m";
    pub const _BLUE: &str = "\x1b[34m";
    pub const _RED: &str = "\x1b[31m";
    pub const _RESET: &str = "\x1B[0m";
}

pub use self::colors::_GREEN as green;
pub use self::colors::_BLUE as blue;
pub use self::colors::_RED as red;
pub use self::colors::_RESET as reset;