use std::path::Path;

// minecraft instance
pub struct Minecraft
{
    pub client: Client,
    pub version: String,
    pub local: Path,
}


// implementations of minecraft
#[derive(Debug)]
pub enum Client
{
    // mojang & microsoft official
    Vanilla,

    // lunar client
    Lunar,

    // fabric minecraft
    Fabric,

    // clip minecraft
    Clip,

    // neo-forge
    NeoForge,

    // forge loader
    Forge,

    // badlion client
    Badlion,

    // liteLoader
    LiteLoader,

    // Quilt
    Quilt,

    // custom client
    // # parameters
    // - [1]`String` client name
    Custom(String)   
}