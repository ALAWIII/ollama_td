use std::fmt::{self, Display};

#[derive(Debug)]
/// The linux platform , either the tool you request is on ```Arm``` or ```X86``` architecture.
pub enum Linux {
    Arm(LinuxArm),
    X86 { rocm: bool },
}

#[derive(Debug)]
pub enum LinuxArm {
    Jetpack5,
    Jetpack6,
    Arm,
}

impl Display for LinuxArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let archi = match self {
            Self::Arm => "arm64.tgz",
            Self::Jetpack5 => "jetpack5.tgz",
            Self::Jetpack6 => "jetpack6.tgz",
        };
        write!(f, "{}", archi)
    }
}
impl Display for Linux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arch = match self {
            Self::Arm(l) => l.to_string(),
            Self::X86 { rocm } => is_rocm(*rocm),
        };
        write!(f, "{}", arch)
    }
}

fn is_rocm(rocm: bool) -> String {
    if rocm {
        return "amd64-rocm.tgz".to_string();
    }
    "amd64.tgz".to_string()
}
