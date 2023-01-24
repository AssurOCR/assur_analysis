pub mod preprocess;

use std::ffi::CString;
use std::ffi::c_double;
use std::os::raw::c_char;
use std::slice;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;



use image::{GrayImage, Luma, DynamicImage};
use image::{error::ImageResult, open, Rgb};
use imageproc::geometric_transformations::{warp, Interpolation, Projection};
use imageproc::contrast::threshold;
use std::cmp::Ordering;
use std::f64::consts::PI;


use core::ffi::CStr;

#[repr(C)]
struct CharacterInfo {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    character: *const c_char,
    conf: f32,
}

#[repr(C)]
struct Page {
    location: *const c_char,
    data: *const c_char,
    n_characters: i32,
    characters: *mut CharacterInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SanatizedCharacterInfo {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub  character: String,
    pub conf: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SanatizedPage {
    pub location: String,
    pub data: String,
    pub n_characters: i32,
    pub characters: Vec<SanatizedCharacterInfo>
}

#[link(name = "smalltess")]
extern {
    fn ocr_pages(pages: *const *const c_char, n_pages: i32, langs: *const c_char) -> *mut Page;
    fn free_pages(pages: *mut Page, total_page: i32);
    fn deskew(image_file: *const c_char, dest_file: *const c_char) -> c_double;

}



fn sanatize_pages(pages: *mut Page, n_pages: usize) -> Vec<SanatizedPage> {
    let mut sanatized_pages = Vec::new();

    unsafe {
        for i in 0..n_pages {
            let page = &*pages.add(i);
            let location = match CStr::from_ptr(page.location).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => String::new(),
            };
            let data = match CStr::from_ptr(page.data).to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => String::new(),
            };
            let characters = slice::from_raw_parts(page.characters, page.n_characters as usize);
            let mut sanatized_characters = Vec::new();
            for character in characters {
                let char_str = match CStr::from_ptr(character.character).to_str() {
                    Ok(s) => s.to_owned(),
                    Err(_) => String::new(),
                };
                sanatized_characters.push(SanatizedCharacterInfo {
                    x1: character.x1,
                    y1: character.y1,
                    x2: character.x2,
                    y2: character.y2,
                    character: char_str,
                    conf: character.conf.clone(),
                });
            }
            sanatized_pages.push(SanatizedPage {
                location,
                data,
                n_characters: page.n_characters,
                characters: sanatized_characters,
            });
        }
    }

    sanatized_pages
}


pub fn recognise_pages(pages: Vec<String>, language: String) -> Vec<SanatizedPage> {
    let c_pages: Vec<CString> = pages
    .iter()
        .map(|s| CString::new(s.as_bytes()).unwrap())
        .collect();
    let c_pages_ptrs: Vec<*const c_char> = c_pages.iter().map(|s| s.as_ptr()).collect();
    let c_pages_ptr_ptr: *const *const c_char = c_pages_ptrs.as_ptr();

    let c_langs = CString::new(language.as_bytes()).unwrap();
    let c_langs_ptr = c_langs.as_ptr();

    let recognized_pages = unsafe { ocr_pages(c_pages_ptr_ptr, pages.len() as i32, c_langs_ptr) };

    let rust_friendly_pages = sanatize_pages(recognized_pages, pages.len());
    unsafe {
        free_pages(recognized_pages, pages.len() as i32);
    }

    rust_friendly_pages
}

pub fn save_json(pages: &Vec<SanatizedPage>, path: &str) {
    let path = Path::new(path);
    let mut file_path = path.to_path_buf();
    if file_path.extension().is_none() {
        file_path.set_extension("json");
    }

    let json_string = serde_json::to_string(&pages).unwrap();

    match fs::write(&file_path, json_string) {
        Ok(_) => println!("Successfully wrote JSON to {}", file_path.to_string_lossy()),
        Err(e) => println!("Error writing JSON to {}: {}", file_path.to_string_lossy(), e),
    }
}


pub fn deskew_save(filename: &str, destination: &str) -> f64 {
    let c_filename = CString::new(filename.as_bytes()).unwrap();
    let c_destination = CString::new(destination.as_bytes()).unwrap();

    unsafe { deskew(c_filename.as_ptr(), c_destination.as_ptr()) }
}



