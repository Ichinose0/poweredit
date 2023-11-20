use std::ptr::{null,null_mut};

use gdiplus_sys2::*;
use raw_window_handle::HasWindowHandle;
use winapi::{um::{winuser::GetDC, wingdi::{CreateSolidBrush, RGB}}, shared::minwindef::FALSE};

#[derive(Debug)]
pub struct CDE {
    token: usize,
    input: GdiplusStartupInput,
    hwnd: isize,
}

impl CDE {
    pub fn new(handle: &impl HasWindowHandle) -> Self {
        let handle = handle.window_handle().unwrap();
        match handle.as_raw() {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                unsafe {
                    let mut token = 0;
                    let input = GdiplusStartupInput {
                        GdiplusVersion: 1,
                        DebugEventCallback: None,
                        SuppressBackgroundThread: FALSE,
                        SuppressExternalCodecs: FALSE,
                    };
                    let status = GdiplusStartup(&mut token, &input, null_mut());
                    println!("{}",status);
                    if status != Status_Ok {
                        panic!("Can't startup GDI+");
                    }
        
                    Self {
                        token,
                        input,
                        hwnd:handle.hwnd.into(),
                    }
                }
            },
            _ => panic!("Error"),
        }    
    }

    pub fn draw(&self) {
        unsafe {
            let mut ps: winapi::um::winuser::PAINTSTRUCT = std::mem::zeroed();
            let hdc = winapi::um::winuser::BeginPaint(self.hwnd as HWND, &mut ps);
            let mut graphics = null_mut();
            let status = gdiplus_sys2::GdipCreateFromHDC(hdc, &mut graphics);
            let mut brush = null_mut();
            gdiplus_sys2::GdipCreateSolidFill(rgb_to_argb(255,0,125,255),&mut brush);
            GdipFillRectangleI(graphics,brush as *mut GpBrush,ps.rcPaint.left as i32,
                ps.rcPaint.top as i32,
                (ps.rcPaint.right - ps.rcPaint.left) as i32,
                (ps.rcPaint.bottom - ps.rcPaint.top) as i32);
                GdipDeleteGraphics(graphics);
                GdipDeleteBrush(brush as *mut GpBrush);
            winapi::um::winuser::EndPaint(self.hwnd as HWND, &ps);
        }
    }
}

impl Drop for CDE {
    fn drop(&mut self) {
        unsafe {
            GdiplusShutdown(self.token);
        }
    }
}

fn rgb_to_argb(alpha: u8, red: u8, green: u8, blue: u8) -> u32 {
    ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}