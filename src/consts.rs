pub const SHC_BACKEND_API_BASE_URL: &str = "https://shc-backend-production.up.railway.app";

// what is the meaning of &str? and what is the meaning of pub?
// &str -> It’s like pointing to a piece of text without owning it, meaning you can use it, but you can’t modify the original string. ✅
// pub -> It’s like saying “Hey, this thing is public, so you can use it from outside this module!” ✅
pub const SHC_CLI_FOLDER_NAME: &str = ".shc-cli";
pub const USER_CONFIG_FILE_NAME: &str = "config.toml";
pub const SHC_IGNORE_FILE_NAME: &str = ".shcignore";

pub const MAX_NAME_WIDTH_LENGTH: usize = 50;

