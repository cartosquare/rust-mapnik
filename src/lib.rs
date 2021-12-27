#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("../bindings.rs");

use std::ffi::CString;
use std::ffi::CStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            println!("mapnik_bbox: {:?}", mapnik_bbox(0.0, 0.0, 90.0, 90.5));
            assert_eq!(1, 1);
        }
    }

    #[test]
    fn create_map() {
        unsafe {
            let input_plugin = CString::new("/usr/local/lib/mapnik/input").unwrap();
            let mut s: *mut std::os::raw::c_char = std::ptr::null_mut();
            mapnik_register_datasources(input_plugin.as_ptr(), &mut s);

            println!("loading input plugins: {:?}", s);

            let fonts = CString::new("/usr/local/lib/mapnik/fonts").unwrap();
            mapnik_register_fonts(fonts.as_ptr(), &mut s);
            println!("loading fonts: {:?}", s);


            let map = mapnik_map(512, 512);
            println!("create map success");

            let xml = CString::new("./styles/test.xml").unwrap();

            mapnik_map_load(map, xml.as_ptr());
            println!("load map success");

            let err = mapnik_map_last_error(map);
            if !err.is_null() {
                println!("load map fail");
            }

            mapnik_map_zoom_all(map);

            let output = CString::new("./output.png").unwrap();
            mapnik_map_render_to_file(map, output.as_ptr());
            println!("render file: {:?}", mapnik_map_last_error(map));

            mapnik_map_free(map);
        }
    }
}
