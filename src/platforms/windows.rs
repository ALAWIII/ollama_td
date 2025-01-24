use std::fmt;

#[derive(Debug)]
/// three architecture options for the Windows platform ```{X86,Arm ,BinExe}``` , the later one is the excutable binary for windows!!!
pub enum Windows {
    X86,
    Arm,
    BinExe,
}
impl fmt::Display for Windows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arch = match self {
            Self::Arm => "arm64.zip",
            Self::X86 => "amd64.zip",
            Self::BinExe => "exe",
        };
        write!(f, "{}", arch)
    }
}
