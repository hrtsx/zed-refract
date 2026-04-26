use zed_extension_api::{
    self as zed, serde_json, Architecture, DownloadedFileType, GithubReleaseOptions,
    LanguageServerId, Os, Result, settings::LspSettings,
};

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
        // 1. User-configured path
        if let Some(path) = LspSettings::for_worktree("refract", worktree)
            .ok()
            .and_then(|s| s.binary)
            .and_then(|b| b.path)
        {
            return Ok(zed::Command { command: path, args: vec![], env: Default::default() });
        }

        // 2. Already cached from a previous download
        if let Some(cached) = &self.cached_binary {
            return Ok(zed::Command {
                command: cached.clone(),
                args: vec![],
                env: Default::default(),
            });
        }

        // 3. Binary in PATH (user installed manually)
        if let Some(path) = worktree.which("refract") {
            return Ok(zed::Command { command: path, args: vec![], env: Default::default() });
        }

        // 4. Download from GitHub releases
        let release = zed::latest_github_release(
            "hrtsx/refract",
            GithubReleaseOptions { require_assets: true, pre_release: false },
        )?;

        let asset_name = asset_name()?;
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| format!("no asset '{asset_name}' in release {}", release.version))?;

        let binary_path = format!("refract-{}", release.version);
        zed::download_file(&asset.download_url, &binary_path, DownloadedFileType::Uncompressed)?;
        zed::make_file_executable(&binary_path)?;

        self.cached_binary = Some(binary_path.clone());
        Ok(zed::Command { command: binary_path, args: vec![], env: Default::default() })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        LspSettings::for_worktree("refract", worktree)
            .map(|s| s.initialization_options)
    }
}

fn asset_name() -> Result<String> {
    let (os, arch) = zed::current_platform();
    let os_str = match os {
        Os::Mac => "macos",
        Os::Linux => "linux",
        Os::Windows => return Err("Windows is not supported".into()),
    };
    let arch_str = match arch {
        Architecture::Aarch64 => "aarch64",
        Architecture::X8664 => "x86_64",
        other => return Err(format!("unsupported architecture: {other:?}").into()),
    };
    Ok(format!("refract-{arch_str}-{os_str}"))
}

zed::register_extension!(RefractExtension);
