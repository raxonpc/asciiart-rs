use leptonica_sys::{ PIX, pixRead, pixDestroy, pixGetWidth, pixGetHeight, pixConvertTo8, pixGetPixel, pixScaleToSize };
use std::ffi::CString;
use crate::asciiart::ProcessingError::{CannotConvertToGrayscale, CannotGetPixel, InvalidFile, UnsupportedImageFormat};

#[derive(Debug)]
pub enum ProcessingError {
    InvalidFile,
    CannotConvertToGrayscale,
    UnsupportedImageFormat,
    CannotScale,
    CannotGetPixel
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ProcessingError::*;
        match *self {
            InvalidFile => write!(f, "File not found or file is not an image"),
            CannotConvertToGrayscale => write!(f, "Could not convert file to grayscale"),
            UnsupportedImageFormat => write!(f, "File is not supported"),
            CannotScale => write!(f, "Could not scale the image"),
            CannotGetPixel=> write!(f, "Could not extract a pixel from the image"),
        }
    }
}

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

fn grayscale_to_ascii(gray: u32) -> char {
    const ASCII_SHADES: &str = " .:-=+*#%@";

    let part = gray as f64 / 255.0;
    let index = part * (ASCII_SHADES.len() - 1) as f64;

    ASCII_SHADES.chars().nth(index as usize).unwrap()
}

impl PixWrapper {
    fn new(file_name: &str) -> Result<Self, ProcessingError> {
        unsafe {
            let c_str = CString::new(file_name).unwrap();

            let pix = pixRead(c_str.as_ptr());
            if pix.is_null() {
                Err(InvalidFile)
            } else {
                Ok(PixWrapper { pix })
            }
        }
    }

    fn to_grayscale(&mut self) -> Result<(), ProcessingError> {
        unsafe {
            match (*self.pix).d {
                1 | 2 | 4 | 8 | 16 | 32 => {
                    let pix = pixConvertTo8(self.pix, 0);
                    if pix.is_null() {
                        Err(CannotConvertToGrayscale)
                    } else {
                        pixDestroy(&mut self.pix);
                        self.pix = pix;
                        Ok(())
                    }
                },
                _ => Err(UnsupportedImageFormat)
            }
        }
    }

    fn scale(&mut self, ratio: f64) -> Result<(), ProcessingError> {
        unsafe {
            let pix = pixScaleToSize(self.pix,
                                     (pixGetWidth(self.pix) as f64 * ratio) as i32,
                                     (pixGetHeight(self.pix)as f64 * ratio) as i32);
            if pix.is_null() {
                Err(ProcessingError::CannotScale)
            } else {
                pixDestroy(&mut self.pix);
                self.pix = pix;
                Ok(())
            }
        }
    }

    fn to_string(&self) -> Result<String, ProcessingError> {
        let mut output = String::new();

        unsafe {
            let width = pixGetWidth(self.pix);
            let height = pixGetHeight(self.pix);
            output.reserve_exact((width * height) as usize);

            for row in 0..height {
                for column in 0..width {
                    let mut out_var: u32 = 0;
                    let status = pixGetPixel(self.pix, column as i32, row as i32, std::ptr::addr_of_mut!(out_var));
                    if status == 1 {
                        return Err(CannotGetPixel)
                    }

                    output.push(grayscale_to_ascii(out_var));
                }
                output.push('\n');
            }
            Ok(output)
        }
    }
}

pub fn to_ascii_art(file_name: &str, scale: Option<f64>) -> Result<String, ProcessingError> {
    let mut wrapper = PixWrapper::new(file_name)?;
    wrapper.to_grayscale()?;
    match scale {
        Some(val) => wrapper.scale(val)?,
        None => wrapper.scale(1.0)?
    }

    wrapper.to_string()
}