#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate photon_rs;
use photon_rs::native;

#[tauri::command]
fn edit_image(image: &str, path: &str) -> () {
  // convert path file to photonImage
  let mut test_img = native::open_image(image).unwrap();

  // edit
  photon_rs::channels::alter_red_channel(&mut test_img, 40);

  let full_str = path.to_owned() + "new_image.png";
  native::save_image(test_img, &full_str);
}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![edit_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


