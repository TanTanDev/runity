use bevy::ecs::schedule::StageLabel;

/// The names of the default App stages
///
/// The relative stages are added by [`App::add_default_stages`].
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    /// Runs only once at the beginning of the app.
    ///
    /// Consists of the sub-stages defined in [`StartupStage`]. Systems added here are
    /// referred to as "startup systems".
    Startup,
    /// Name of app stage that runs before all other app stages
    First,
    /// Name of app stage responsible for performing setup before an update. Runs before UPDATE.
    PreUpdate,
    /// Name of app stage responsible for doing most app logic. Systems should be registered here
    /// by default.
    Update,
    /// Name of app stage responsible for processing the results of UPDATE. Runs after UPDATE.
    PostUpdate,
    /// Name of app stage that runs after all other app stages
    Last,
    /// the last stage is to send to unity
    /// calls into unity must be single threaded, very important
    UploadToUnity,
    LateUploadToUnity,
}