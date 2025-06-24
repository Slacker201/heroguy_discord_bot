

pub fn extract_string_option(opt: Option<&serenity::all::CommandDataOption>) -> Option<&str> {
    match opt {
        Some(var) => var.value.as_str(),
        None => None,
    }
}