// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn search(input: &str) -> String {
    format!("You searched {}.", input)
}

fn setup_menu(app_name: &String) -> Menu {
    let mut menu = Menu::new();

    // ========================================================================
    // APP MENU (macOS only)
    // ========================================================================
    #[cfg(target_os = "macos")]
    {
        menu = menu.add_submenu(Submenu::new(
            app_name,
            Menu::new()
                .add_native_item(MenuItem::About(
                    app_name.to_string(),
                    AboutMetadata::default(),
                ))
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
        CustomMenuItem::new("newFile".to_string(), "New File...").accelerator("CmdOrCtrl+N");
    let open_file =
        CustomMenuItem::new("openFile".to_string(), "Open File...").accelerator("CmdOrCtrl+O");
    let new_transaction = CustomMenuItem::new("newTransaction".to_string(), "New Transaction...")
        .accelerator("CmdOrCtrl+T");
    let new_account = CustomMenuItem::new("newAccount".to_string(), "New Account...")
        .accelerator("CmdOrCtrl+Shift+T");

    file_menu = file_menu
        .add_item(new_file)
        .add_item(open_file)
        .add_item(new_account)
        .add_item(new_transaction);
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

fn setup_menu_event_handler(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "newFile" => {
            println!("File -> New File menu item clicked!");
        }
        "openFile" => {
            println!("File -> Open File menu item clicked!");
        }
        "newAccount" => {
            println!("File -> New Account menu item clicked!");
        }
        "newTransaction" => {
            println!("File -> New Transaction menu item clicked!");
        }
        _ => {}
    }
}

fn main() {
    let context = tauri::generate_context!();
    let app_name = context.package_info().name.clone();
    let menu = setup_menu(&app_name);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search])
        .menu(menu)
        .on_menu_event(|event| setup_menu_event_handler(event))
        .run(context)
        .expect("error while running tauri application");
}
