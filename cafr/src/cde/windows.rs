use std::ptr::{null,null_mut};

use gdiplus_sys2::*;
use raw_window_handle::HasWindowHandle;
use winapi::{um::{winuser::GetDC, wingdi::{CreateSolidBrush, RGB}}, shared::minwindef::FALSE};

use crate::widget::Target;

#[derive(Debug)]
pub struct CDE<T> 
where
    T: Send + std::fmt::Debug
{
    token: usize,
    input: GdiplusStartupInput,
    hwnd: isize,
    msg: Option<T>
}

impl<T> CDE<T> 
where
    T: Send + std::fmt::Debug
{
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
                        msg: None
                    }
                }
            },
            _ => panic!("Error"),
        }    
    }

    pub fn draw(&self,color: crate::Color,target: &Target<T>) {
        unsafe {
            let mut ps: winapi::um::winuser::PAINTSTRUCT = std::mem::zeroed();
            let hdc = winapi::um::winuser::BeginPaint(self.hwnd as HWND, &mut ps);
            let mut graphics = null_mut();
            let status = gdiplus_sys2::GdipCreateFromHDC(hdc, &mut graphics);

            //Drawing start

            self.draw_background(color,&ps,graphics);

            for i in target.get() {
                let mut brush = null_mut();
                gdiplus_sys2::GdipCreateSolidFill(color_to_argb(i.color()),&mut brush);
                let shadow = i.shadow();
                let mut blur = null_mut();
                gdiplus_sys2::GdipCreateSolidFill(color_to_argb(shadow.color),&mut blur);
                GdipFillRectangleI(graphics,blur as *mut GpBrush,(30-shadow.border) as i32,
                    (30-shadow.border) as i32,
                (i.width()+shadow.border*2) as i32,
                (i.height()+shadow.border*2) as i32);
                GdipFillRectangleI(graphics,brush as *mut GpBrush,30,
                    30,
                    i.width() as i32,
                    i.height() as i32);
                    GdipDeleteBrush(blur as *mut GpBrush);
                    GdipDeleteBrush(brush as *mut GpBrush);
            }
            GdipDeleteGraphics(graphics);
            winapi::um::winuser::EndPaint(self.hwnd as HWND,&ps);
        }
    }

    fn draw_background(&self,color: crate::Color,ps: &winapi::um::winuser::PAINTSTRUCT,graphics: *mut GpGraphics) {
        unsafe {
            let mut brush = null_mut();
            
            gdiplus_sys2::GdipCreateSolidFill(color_to_argb(color),&mut brush);
            GdipFillRectangleI(graphics,brush as *mut GpBrush,ps.rcPaint.left as i32,
                ps.rcPaint.top as i32,
                (ps.rcPaint.right - ps.rcPaint.left) as i32,
                (ps.rcPaint.bottom - ps.rcPaint.top) as i32);
                GdipDeleteBrush(brush as *mut GpBrush);
        }
    }
}

impl<T> Drop for CDE<T> 
where
    T: Send + std::fmt::Debug
{
    fn drop(&mut self) {
        unsafe {
            GdiplusShutdown(self.token);
        }
    }
}

fn rgb_to_argb(alpha: u8, red: u8, green: u8, blue: u8) -> u32 {
    ((alpha as u32) << 24) | ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32)
}

fn color_to_argb(color: crate::Color) -> u32 {
    match color {
        crate::Color::Black => rgb_to_argb(255,0,0,0),
        crate::Color::White => rgb_to_argb(255,255,255,255),
        crate::Color::ARGB(a, r, g, b) => rgb_to_argb(a,r,g,b),
    }
}