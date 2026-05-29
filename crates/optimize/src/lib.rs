pub struct OptimizeArgs<'a> {
    pub input: &'a std::path::Path,
    pub output: Option<&'a std::path::Path>,
    pub quality: u8,
    pub keep_original: bool,
}
pub fn run(_args: OptimizeArgs) -> Result<(), String> { Ok(()) }
