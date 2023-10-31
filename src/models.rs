#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operation {
    Pack,
    Cook,
}

pub struct CookOptions {
    pub target_platform: String,
    pub iterate: bool,
    pub map: Vec<String>,
    pub cook_on_the_fly: bool,
    pub map_ini_section: Option<String>,
    pub un_versioned: bool,
    pub cook_all: bool,
    pub compressed: bool,
}
