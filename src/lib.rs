use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json};

struct RefractExtension {
    cached_binary: Option<String>,
}

impl zed::Extension for RefractExtension {
    fn new() -> Self {
        Self { cached_binary: None }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Use user-configured path if set
        let settings = zed::LanguageServerSettings::for_worktree("refract", worktree)?;
        if let Some(path) = settings.binary.as_ref().and_then(|b| b.path.as_ref()) {
            return Ok(zed::Command {
                command: path.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        // Check PATH first (user installed manually)
        if let Some(path) = worktree.which("refract") {
            self.cached_binary = Some(path.clone());
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: Default::default(),
            });
        }

        // Download from GitHub releases
        if let Some(cached) = &self.cached_binary {
            return Ok(zed::Command {
                command: cached.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        let release = zed::latest_github_release(
            "hrtsx/refract",
            zed::GithubReleaseOptions { require_assets: true, pre_release: false },
        )?;

        let asset_name = asset_name();
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset {asset_name} in release {}", release.version))?;

        let binary_path = zed::download_file(
            &asset.download_url,
            &format!("refract-{}", release.version),
            zed::DownloadedFileType::Uncompressed,
        )?;

        zed::make_file_executable(&binary_path)?;
        self.cached_binary = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = zed::LanguageServerSettings::for_worktree("refract", worktree)?;
        Ok(settings.initialization_options.clone())
    }
}

fn asset_name() -> String {
    let os = match std::env::consts::OS {
        "macos" => "macos",
        _ => "linux",
    };
    let arch = match std::env::consts::ARCH {
        "aarch64" => "aarch64",
        _ => "x86_64",
    };
    format!("refract-{arch}-{os}")
}

zed::register_extension!(RefractExtension);
