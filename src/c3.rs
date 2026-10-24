use std::fs;
use zed_extension_api::{
    self as zed, current_platform, download_file, latest_github_release, make_file_executable,
    set_language_server_installation_status, Architecture, DownloadedFileType, GithubReleaseOptions,
    LanguageServerId, LanguageServerInstallationStatus, Os, Result,
};

struct C3Extension {
    cached_binary_path: Option<String>,
}

impl C3Extension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. Prefer binary from PATH if present
        if let Some(path) = worktree.which("c3_ls") {
            return Ok(path);
        }
        if let Some(path) = worktree.which("c3lsp") {
            return Ok(path);
        }

        // 2. Return cached binary if valid
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // 3. Resolve latest release from GitHub
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = latest_github_release(
            "ManuLinares/c3_ls",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = current_platform();

        let (os_str, arch_str, file_type, binary_name) = match (platform, arch) {
            (Os::Linux, Architecture::X8664) => {
                ("linux", "x86_64", DownloadedFileType::GzipTar, "c3_ls")
            }
            (Os::Linux, Architecture::Aarch64) => {
                ("linux", "aarch64", DownloadedFileType::GzipTar, "c3_ls")
            }
            (Os::Mac, Architecture::X8664) => {
                ("macos", "x86_64", DownloadedFileType::GzipTar, "c3_ls")
            }
            (Os::Mac, Architecture::Aarch64) => {
                ("macos", "aarch64", DownloadedFileType::GzipTar, "c3_ls")
            }
            (Os::Windows, Architecture::X8664) => {
                ("windows", "x86_64", DownloadedFileType::Zip, "c3_ls.exe")
            }
            _ => {
                return Err(format!(
                    "unsupported architecture/OS combination: {platform:?} {arch:?}"
                ))
            }
        };

        let extension = match file_type {
            DownloadedFileType::GzipTar => "tar.gz",
            DownloadedFileType::Zip => "zip",
            _ => return Err("unsupported archive type".into()),
        };

        let asset_name = format!("c3_ls-{os_str}-{arch_str}.{extension}");

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("c3_ls-{}", release.version);
        let binary_path = format!("{version_dir}/{binary_name}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            set_language_server_installation_status(
                language_server_id,
                &LanguageServerInstallationStatus::Downloading,
            );

            download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|e| format!("failed to download file: {e}"))?;

            make_file_executable(&binary_path)?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for C3Extension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(C3Extension);