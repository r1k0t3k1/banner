mod characters;

pub fn banner(s: &str) -> Result<(), std::io::Error> {
    let characters = characters::string_to_banner_char(s).unwrap();
    characters::render_chars(characters);
    Ok(())
}
