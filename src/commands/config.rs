use seahorse::Command;

pub fn config() -> Command {
    Command::new("config")
        .description("Clone project configuration file(s) to the current working directory")
        .action(|c| {})
}
