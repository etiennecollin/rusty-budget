// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusty_budget::{structs::AppState, utils::*};
use tauri::{
    api::dialog, AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu,
    WindowMenuEvent,
};

fn setup_menu(app_name: String) -> Menu {
    let mut menu = Menu::new();

    // ========================================================================
    // APP MENU (macOS only)
    // ========================================================================
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name.clone(),
            Menu::new()
                .add_native_item(MenuItem::About(app_name, AboutMetadata::default()))
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Services)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Hide)
                .add_native_item(MenuItem::HideOthers)
                .add_native_item(MenuItem::ShowAll)
                .add_native_item(MenuItem::Separator)
                .add_native_item(MenuItem::Quit),
        ));
    }

    // ========================================================================
    // FILE MENU ==============================================================
    // ========================================================================
    let mut file_menu = Menu::new();

    // ========================================================================
    // CUSTOM FILE MENU ITEMS =================================================
    // ========================================================================
    let new_file =
        CustomMenuItem::new("newFile".to_owned(), "New File...").accelerator("CmdOrCtrl+N");
    let open_file =
        CustomMenuItem::new("openFile".to_owned(), "Open File...").accelerator("CmdOrCtrl+O");

    file_menu = file_menu.add_item(new_file).add_item(open_file);
    // ========================================================================

    file_menu = file_menu.add_native_item(MenuItem::CloseWindow);
    #[cfg(not(target_os = "macos"))]
    {
        file_menu = file_menu.add_native_item(MenuItem::Quit);
    }
    menu = menu.add_submenu(Submenu::new("File", file_menu));

    // ========================================================================
    // EDIT MENU ==============================================================
    // ========================================================================
    #[cfg(not(target_os = "linux"))]
    let mut edit_menu = Menu::new();
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Undo);
        edit_menu = edit_menu.add_native_item(MenuItem::Redo);
        edit_menu = edit_menu.add_native_item(MenuItem::Separator);
    }
    #[cfg(not(target_os = "linux"))]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::Cut);
        edit_menu = edit_menu.add_native_item(MenuItem::Copy);
        edit_menu = edit_menu.add_native_item(MenuItem::Paste);
    }
    #[cfg(target_os = "macos")]
    {
        edit_menu = edit_menu.add_native_item(MenuItem::SelectAll);
    }
    #[cfg(not(target_os = "linux"))]
    {
        menu = menu.add_submenu(Submenu::new("Edit", edit_menu));
    }

    // ========================================================================
    // VIEW MENU (maxOS only) =================================================
    // ========================================================================
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            "View",
            Menu::new().add_native_item(MenuItem::EnterFullScreen),
        ));
    }

    // ========================================================================
    // WINDOW MENU ============================================================
    // ========================================================================
    let mut window_menu = Menu::new();
    window_menu = window_menu.add_native_item(MenuItem::Minimize);
    #[cfg(target_os = "macos")]
    {
        window_menu = window_menu.add_native_item(MenuItem::Zoom);
        window_menu = window_menu.add_native_item(MenuItem::Separator);
    }
    window_menu = window_menu.add_native_item(MenuItem::CloseWindow);
    menu = menu.add_submenu(Submenu::new("Window", window_menu));

    // ========================================================================
    menu
}

#[tauri::command]
fn setup_menu_event_handler(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "newFile" => {
            println!("File -> New File menu item clicked!");
            // let window = event.window();
            // let window_name = window.label().to_string();
            // let app = window.app_handle().windows()[window_name.as_str()];

            // Send event to the frontend to open file saving dialog
        }
        "openFile" => {
            println!("File -> Open File menu item clicked!");
            dialog::FileDialogBuilder::default()
                .add_filter("RustyBudget", &["rsb"])
                .pick_file(|path_buf| match path_buf {
                    Some(p) => {}
                    _ => {}
                });
        }
        _ => {}
    }
}

fn main() {
    let context = tauri::generate_context!();
    let app_name = context.package_info().name.clone();
    let menu = setup_menu(app_name);

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            search,
            get_transaction_categories,
            get_transactions,
            get_currency_symbols,
            get_accounts,
            is_file_loaded,
            is_account_loaded,
            new_file,
            add_transaction,
            save_file
        ])
        .menu(menu)
        .on_menu_event(|event| setup_menu_event_handler(event))
        .run(context)
        .expect("error while running tauri application");
}
