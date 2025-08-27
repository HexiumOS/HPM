pub struct Version {
    epoch: u32,
    major: u32,
    minor: u32,
    patch: u32,
    compatibility: Compatibity,
}

pub enum Compatibity {
    None,
    Epoch,
    Major,
    Minor,
    Patch,
}
