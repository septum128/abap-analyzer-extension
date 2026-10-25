use zed_extension_api::{self as zed, LanguageServerId, Result};

struct AbapExtension;

impl zed::Extension for AbapExtension {
    fn new() -> Self {
        AbapExtension
    }

    // ZedがLSPを起動するときに呼ばれる: 実行するコマンドを返す
    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // PATH上の abap-language-server を探す
        let path = worktree
            .which("abap-language-server")
            .ok_or_else(|| "abap-language-server が PATH に見つかりません".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(AbapExtension);
