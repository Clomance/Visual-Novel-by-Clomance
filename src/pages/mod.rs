mod main_menu;
pub use main_menu::MainMenu;

mod loading_screen;
pub use loading_screen::LoadingScreen;

mod enter_user_name;
pub use enter_user_name::EnterUserName;

mod pause_menu;
pub use pause_menu::PauseMenu;

mod settings;
pub use settings::SettingsPage;

mod intro;
pub use intro::Intro;

pub const default_page_smooth:f32=1f32/16f32; // 1 к количеству кадров перехода