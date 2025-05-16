// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Staff {
    pub name: String,
    pub avatar: String,
    pub total_appointments: i32,
}

#[tauri::command]
fn get_staff_list() -> Vec<Staff> {
    vec![
        Staff {
            name: "John Doe".to_string(),
            avatar: "https://i.pravatar.cc/150?img=1".to_string(),
            total_appointments: 15,
        },
        Staff {
            name: "Jane Smith".to_string(),
            avatar: "https://i.pravatar.cc/150?img=2".to_string(),
            total_appointments: 23,
        },
        Staff {
            name: "Mike Johnson".to_string(),
            avatar: "https://i.pravatar.cc/150?img=3".to_string(),
            total_appointments: 8,
        },
        Staff {
            name: "Sarah Williams".to_string(),
            avatar: "https://i.pravatar.cc/150?img=4".to_string(),
            total_appointments: 31,
        },
        Staff {
            name: "David Brown".to_string(),
            avatar: "https://i.pravatar.cc/150?img=5".to_string(),
            total_appointments: 19,
        },
        Staff {
            name: "Emily Davis".to_string(),
            avatar: "https://i.pravatar.cc/150?img=6".to_string(),
            total_appointments: 27,
        },
        Staff {
            name: "Robert Wilson".to_string(),
            avatar: "https://i.pravatar.cc/150?img=7".to_string(),
            total_appointments: 12,
        },
        Staff {
            name: "Lisa Anderson".to_string(),
            avatar: "https://i.pravatar.cc/150?img=8".to_string(),
            total_appointments: 34,
        },
        Staff {
            name: "James Taylor".to_string(),
            avatar: "https://i.pravatar.cc/150?img=9".to_string(),
            total_appointments: 21,
        },
        Staff {
            name: "Emma Martinez".to_string(),
            avatar: "https://i.pravatar.cc/150?img=10".to_string(),
            total_appointments: 16,
        },
    ]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_staff_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
