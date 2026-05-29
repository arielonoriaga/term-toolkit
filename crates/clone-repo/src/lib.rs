pub struct CloneArgs<'a> {
    pub url: &'a str,
    pub output: Option<&'a std::path::Path>,
    pub reset_history: bool,
}
pub fn run(_args: CloneArgs) -> Result<(), String> { Ok(()) }
