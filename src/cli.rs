use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("shc")
        .about("share code in minimum time")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .allow_external_subcommands(true)
        .subcommand(Command::new("login").about("login to use shc"))
        .subcommand(
            Command::new("add")
                .about("upload file")
                .arg(arg!(<FILE> "file path to upload"))
                .arg_required_else_help(false),
        )
        .subcommand(
            Command::new("list")
                .about("list all files")
                .arg(arg!(<FILTER> "filter by filename").required(false)),
        )
        .subcommand(
            Command::new("remove")
                .about("remove file")
                .arg(arg!(<FILTER> "filter by filename").required(false)),
        )
        .subcommand(
            Command::new("visibility")
                .about("toggle file's visibility")
                .arg(arg!(<FILTER> "filter by filename").required(false)),
        )
        .subcommand(
            Command::new("rename")
                .about("rename file")
                .arg(arg!(<FILTER> "filter by filename").required(false)),
        )
        .subcommand(
            Command::new("get")
                .about("download file")
                .arg(arg!(<FILTER> "filter by filename").required(false)),
        )
        .subcommand(Command::new("logout").about("logout from shc"))
}
