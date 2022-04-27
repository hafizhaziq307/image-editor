use image::{image_dimensions, open, DynamicImage};
use image::imageops::{resize, FilterType};
use tauri;

#[tauri::command]
pub fn get_image_dimensions(path: &str) -> (u32, u32) {
  let dimension = image_dimensions(path).unwrap();

  return dimension;
}

#[tauri::command]
pub fn edit_image(image_path: &str, queues: &str, download_dir: &str) -> () {
 
  let arr_queues: Vec<&str> = queues.split("::").collect(); 

  let mut img = open(image_path).unwrap();

  for queue in arr_queues {
    let details: Vec<&str> = queue.split(" ").collect();

    let edit_type:&str = details[0];
    let width: u32 = details[1].parse().unwrap();
    let height: u32 = details[2].parse().unwrap();

    if edit_type == "resize"{
      img = DynamicImage::ImageRgba8(resize(&img, width, height, FilterType::Lanczos3));
    }
  }

  let filename = download_dir.to_owned() + "new_image.png";
  img.save(filename).unwrap();
}