use zed::LanguageServerId;
use zed_extension_api::{self as zed, settings::LspSettings, Result};

struct RExtension;

impl zed::Extension for RExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "r_language_server" => {
                let Some(r_exec) = worktree.which("R") else {
                    return Err("R path not found".to_string());
                };
                Ok(zed::Command {
                    command: r_exec.to_string(),
                    args: vec![
                        "--slave".to_string(),
                        "-e".to_string(),
                        "languageserver::run()".to_string(),
                    ],
                    env: Default::default(),
                })
            }
            "air" => {
                let Some(air_exec) = worktree.which("air") else {
                    return Err("air path not found".to_string());
                };
                Ok(zed::Command {
                    command: air_exec.to_string(),
                    args: vec!["language-server".into()],
                    env: Default::default(),
                })
            }
            _ => return Err("Unsupported language server".to_string()),
        }
    }
}

zed::register_extension!(RExtension);
