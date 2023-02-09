use leptonica_sys::{ PIX, pixRead, pixDestroy, pixRemoveColormap, pixConvert16To8, pixConvertRGBToGrayFast, REMOVE_CMAP_TO_GRAYSCALE };
use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::ptr::null;

struct PixWrapper {
    pix: *mut PIX
}

impl Drop for PixWrapper {
    fn drop(&mut self) {
        unsafe {
            pixDestroy(&mut self.pix);
        }
    }
}

impl PixWrapper {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        unsafe {
            let c_str = CString::new(file_name).unwrap();

            unsafe {
                let pix = pixRead(c_str.as_ptr());
                if pix.is_null() {
                    Err(std::io::Error::new(ErrorKind::Other, "Error reading file"))
                } else {
                    Ok(PixWrapper { pix })
                }
            }
        }
    }

    fn to_grayscale(&mut self) -> Result<(), String> {
        unsafe {
            match (*self.pix).d {
                1 | 2 | 4 | 8 => {
                    let pix = pixRemoveColormap(self.pix, REMOVE_CMAP_TO_GRAYSCALE as i32);
                    if pix.is_null() {
                        Err("Failed removing colormap from the image".to_string())
                    } else {
                        pixDestroy(&mut self.pix);
                        self.pix = pix;
                        Ok(())
                    }
                },
                16 => {
                    let mut pix = pixConvert16To8(self.pix, 0);
                    if pix.is_null() {
                        return Err("Failed converting 16bpp image to 8bpp image".to_string());
                    }

                    pixDestroy(&mut pix);
                    let pix = pixRemoveColormap(pix, REMOVE_CMAP_TO_GRAYSCALE as i32);
                    if pix.is_null() {
                        Err("Failed removing colormap from the image".to_string())
                    } else {
                        pixDestroy(&mut self.pix);
                        self.pix = pix;
                        Ok(())
                    }
                },
                32 => {
                    let pix = pixConvertRGBToGrayFast(self.pix);
                    if pix.is_null() {
                        Err("Failed removing colormap from the image".to_string())
                    } else {
                        pixDestroy(&mut self.pix);
                        self.pix = pix;
                        Ok(())
                    }
                }
                _ => todo!()
            }
        }
    }
}

fn read_pix(file_name: &str) -> Result<*mut PIX, std::io::Error> {
    let c_str = CString::new(file_name).unwrap();

    unsafe {
        let pix = pixRead(c_str.as_ptr());
        if pix.is_null() {
            Err(std::io::Error::new(ErrorKind::Other, "Error reading file"))
        } else {
            Ok(pix)
        }
    }
}


pub fn to_ascii_art(file_name: &str) -> String {
    PixWrapper::new(file_name).unwrap().to_grayscale().unwrap();
    "Dupa".to_string()
}