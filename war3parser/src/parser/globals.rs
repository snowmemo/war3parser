/// TRIGSTR_007 / TRIGSTR_007ab / TRIGSTR_7 TRIGSTR_-007
pub const TRAGGER_STR_RE: &str = r"TRIGSTR_(-?\d+)(?:\w+)?";

/// STRING_007
pub const STRINGS_RE: &str = r"STRING\s+([0-9]+)\s+\{\r\n+([^\}]*)\r\n\}";
