mod commands;
mod version;

pub struct Package {
    author: String,
    name: String,
    version: version::Version,

    description: String,
    license: String,

    dependencies: Vec<Package>,
}
