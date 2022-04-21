#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use image::{image_dimensions};
use tauri;

#[tauri::command]
fn get_image_dimensions(path: &str) -> (u32, u32) {
  println!("input: {}", path);

  let dimension = image_dimensions(path).unwrap();

  return dimension;
}


// #[tauri::command]
// fn edit_image(image: &str, path: &str) -> () {
//   // convert path file to photonImage
//   let mut test_img = native::open_image(image).unwrap();
  

//   // edit
//   photon_rs::channels::alter_red_channel(&mut test_img, 40);

//   let full_str = path.to_owned() + "new_image.png";
//   native::save_image(test_img, &full_str);
// }


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_image_dimensions])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


