use std::fmt;

#[derive(Debug)]
/// Either the binary version or the zipped one!!!
pub enum Unix {
    DarwinBin,
    DarwinZip,
}
impl fmt::Display for Unix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = match self {
            Self::DarwinBin => "darwin.tgz",
            Self::DarwinZip => "darwin.zip",
        };
        write!(f, "{}", arch)
    }
}
