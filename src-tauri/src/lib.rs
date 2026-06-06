mod commands;

use commands::{
    config::{get_config_cmd, save_config_cmd, is_initialized_cmd, reset_api_key_cmd},
    email::{complete_invitation, invite_user, send_invite_email},
    litellm::{create_user, generate_invitation, list_users, list_access_groups, test_model},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            is_initialized_cmd,
            get_config_cmd,
            save_config_cmd,
            reset_api_key_cmd,
            create_user,
            generate_invitation,
            list_users,
            list_access_groups,
            test_model,
            send_invite_email,
            complete_invitation,
            invite_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
