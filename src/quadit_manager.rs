use std::fmt;

use anyhow::Ok;
use tracing::instrument;

use crate::{config_quadit::ConfigQuadit, git_manager::GitManager, reload_manager::ReloadManager};
/// The manager of managers responsible for loading config and starting the different schedulers.
pub struct QuaditManager {
    /// The git scheduler
    pub git_manager: GitManager,
    /// The scheduler responsible for loading git commands
    pub reload_manager: Option<ReloadManager>,
}

impl fmt::Debug for QuaditManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quadit Manager")
    }
}

impl QuaditManager {
    /// Returns an configured quadit manager.
    /// # Arguments
    ///
    /// * `conf` - A String slice that contains the complete `config.yaml`
    #[instrument]
    pub async fn from_yaml(conf: String) -> Result<QuaditManager, anyhow::Error> {
        let quad = ConfigQuadit::from_yaml(conf)?;
        if quad.config_reload.is_some() {
            Ok(QuaditManager {
                git_manager: GitManager::from_target_configs(quad.target_configs).await?,
                reload_manager: Some(
                    ReloadManager::from_config_reload(quad.config_reload.unwrap()).await?,
                ),
            })
        } else {
            Ok(QuaditManager {
                git_manager: GitManager::from_target_configs(quad.target_configs).await?,
                reload_manager: None,
            })
        }
    }

    // Need to put this here as it's shared between the schedulers
    // pub fn config_git_list() -> &'static Mutex<HashMap<uuid::Uuid, ConfigGit>> {
    //     static HASHMAP: OnceLock<Mutex<HashMap<uuid::Uuid, ConfigGit>>> = OnceLock::new();
    //     let hm: HashMap<uuid::Uuid, ConfigGit> = HashMap::new();
    //     HASHMAP.get_or_init(|| Mutex::new(hm))
    // }

    /// Starts the scheduler services
    #[instrument]
    pub async fn start(self) -> Result<(), anyhow::Error> {
        self.git_manager.start().await?;
        if self.reload_manager.is_some() {
            self.reload_manager.as_ref().unwrap().start().await?;
        }
        Ok(())
    }
}
