//! A straightforward way to store and retrieve user preferences on disk for Bevy applications.

use bevy::prelude::*;

mod file_system;
pub mod modals;

/// Annotation for a type to show which type of settings it belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum SettingsType {
    /// These are settings that are saved in the os user's configuration directory. \
    /// These settings are global to the user and are not tied to a specific project. \
    /// Settings are along the lines of hotkeys etc.
    Global,
    /// Workspace preferences use the global preferences by default. End users can modify them, customizing their layout, theming and hotkeys. \
    /// The file is created when the user applies changes to their workspace preferences within the editor. \
    /// Workspace preferences can be shared between multiple projects and are not isolated to project folders.*
    Workspace,
    /// Project preference overrides are empty and stored within the project settings. \
    ///  When a project overrides a global/workspace preference, it is no longer possible to change them. \
    ///  In order to modify the preference, users must modify the project settings instead.
    /// There are two states that overrides can be in:
    /// - Inheriting - No override is set. Users can freely change the preference. Users can use what they have set within the global/workspace preferences.
    /// - Modified - When an override has been set, users can no longer change the preference without modifying the project settings. You can switch between inheriting and modified at any time without consequence.
    Project,
}

#[derive(Debug, Clone, Reflect)]
/// Annotation for a type to add tags to the settings. these tags can be used to filter settings in the editor.
pub struct SettingsTags(pub Vec<&'static str>);

#[derive(Resource)]
/// Store the path for the global preferences directory.
pub struct GlobalSettingsPath(pub std::path::PathBuf);

/// A Bevy plugin for editor settings.
/// This plugin loads the workspace settings, user settings, and project settings.
pub struct EditorSettingsPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Resource, Reflect)]
/// Represents the settings for the editor.
/// This includes workspace settings, user settings, and project settings.
pub struct Settings {
    /// Settings for the workspace
    pub workspace_settings: modals::workspace::WorkspaceSettings,
    /// Settings for the user
    pub user_settings: modals::user::UserSettings,
}

impl Plugin for EditorSettingsPlugin {
    fn build(&self, app: &mut App) {
        match file_system::global_settings_path() {
            Some(path) => {
                debug!("Global settings path: {:?}", path);
                app.insert_resource(GlobalSettingsPath(path));
            }
            None => {
                warn!("Failed to load global settings");
            }
        };
    }

    fn finish(&self, app: &mut App) {
        file_system::load_settings(app);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Reflect, Resource)]
    #[reflect(@SettingsType::Project, @SettingsTags(vec!["basic", "settings", "testing"]))]
    struct BasicSettings {
        pub name: String,
        pub age: u32,
    }

    #[test]
    fn loading() {
        let mut app = App::new();

        app.register_type::<BasicSettings>();

        app.insert_resource(BasicSettings {
            name: "John".to_string(),
            age: 25,
        });

        file_system::load_project_settings(app.world_mut());

        let settings = app.world().get_resource::<BasicSettings>().unwrap();

        assert_eq!(settings.name, "bevy_editor_settings");
        assert_eq!(settings.age, 25);
    }
}
