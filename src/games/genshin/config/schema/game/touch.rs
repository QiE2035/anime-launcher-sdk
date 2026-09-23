use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use crate::genshin::consts::launcher_dir;

/// Name of the default touch support folder inside the launcher folder
pub const DEFAULT_FOLDER_NAME: &str = "touch";

/// Name of the Hk4eTouch injector executable
pub const INJECTOR_NAME: &str = "hk4e-touch.exe";

/// Name of the Hk4eTouch hook library expected next to the injector
pub const HOOK_NAME: &str = "hk4e-touch-hook.dll";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Touch {
    /// Switch the game to its mobile UI by attaching the Hk4eTouch injector
    pub enabled: bool,

    /// Folder with the Hk4eTouch injector and hook library
    pub path: PathBuf
}

impl Default for Touch {
    #[inline]
    fn default() -> Self {
        Self {
            enabled: false,
            path: launcher_dir()
                .map(|dir| dir.join(DEFAULT_FOLDER_NAME))
                .unwrap_or_else(|_| PathBuf::from(DEFAULT_FOLDER_NAME))
        }
    }
}

impl Touch {
    /// Path to the injector executable
    #[inline]
    pub fn injector(&self) -> PathBuf {
        self.path.join(INJECTOR_NAME)
    }

    /// Path to the hook library
    #[inline]
    pub fn hook(&self) -> PathBuf {
        self.path.join(HOOK_NAME)
    }

    /// Both the injector and the hook library have to be present
    #[inline]
    pub fn is_installed(&self) -> bool {
        self.injector().is_file() && self.hook().is_file()
    }
}

impl From<&JsonValue> for Touch {
    fn from(value: &JsonValue) -> Self {
        let default = Self::default();

        Self {
            enabled: value.get("enabled")
                .and_then(JsonValue::as_bool)
                .unwrap_or(default.enabled),

            path: value.get("path")
                .and_then(JsonValue::as_str)
                .map(PathBuf::from)
                .unwrap_or(default.path)
        }
    }
}
