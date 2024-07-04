pub mod win32 {
    use winapi::{
        um::{
            winuser::{self, SW_SHOW, WINDOWINFO},
            shellapi::{self, SEE_MASK_NOASYNC, SHELLEXECUTEINFOW},
            winnt::{HANDLE, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
            handleapi,
            processthreadsapi,
            securitybaseapi
        },
        shared::{
            windef::{HWND, RECT},
            minwindef::DWORD
        }
    };
    use std::{
        ffi::OsStr, mem, os::windows::ffi::OsStrExt, ptr
    };

    fn to_wide_str(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect::<Vec<u16>>()
    }

    pub fn start_shell_with_uac(file: &str) -> bool {
        let mut execute_info: SHELLEXECUTEINFOW = unsafe { mem::zeroed() };
        execute_info.cbSize = mem::size_of::<SHELLEXECUTEINFOW>() as u32;
        execute_info.lpFile = to_wide_str(file).as_ptr();
        execute_info.lpVerb = to_wide_str("runas").as_ptr();
        execute_info.nShow = SW_SHOW;
        execute_info.fMask = SEE_MASK_NOASYNC;

        unsafe { shellapi::ShellExecuteExW(&mut execute_info) != 0 }
    }

    pub fn is_elevated() -> bool {
        let mut elevated = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut token_handle: HANDLE = ptr::null_mut();

        unsafe {
            if processthreadsapi::OpenProcessToken(
                processthreadsapi::GetCurrentProcess(), 
                TOKEN_QUERY, 
                &mut token_handle
            ) == 0 {
                return false
            }

            let mut ret_len: DWORD = 0;

            if securitybaseapi::GetTokenInformation(
                token_handle, 
                TokenElevation, 
                &mut elevated as *mut _ as *mut _, 
                mem::size_of_val(&elevated) as DWORD, 
                &mut ret_len
            ) == 0 {
                handleapi::CloseHandle(token_handle);

                return false
            }

            // close handle to avoid memory leak
            handleapi::CloseHandle(token_handle);
        }

        elevated.TokenIsElevated != 0
    }

    pub fn find_window(class_name: Option<&str>, window_name: &str) -> Option<HWND> {
        let mut cls_name = None;
        if let Some(name) = class_name {
            cls_name = Some(to_wide_str(name));
        }
        let window_name = to_wide_str(window_name);

        unsafe {
            let hwnd = winuser::FindWindowW(match cls_name {
                Some(name) => name.as_ptr(),
                None => ptr::null()
            }, window_name.as_ptr());
            
            if hwnd.is_null() {
                None
            }
            else {
                Some(hwnd)
            }
        }
    }

    pub fn get_window_info(hwnd: HWND) -> Option<WINDOWINFO> {
        unsafe {
            let mut window_info = WINDOWINFO {
                cbSize: mem::size_of::<WINDOWINFO>() as u32,
                rcWindow: mem::zeroed(),
                rcClient: mem::zeroed(),
                dwStyle: 0,
                dwExStyle: 0,
                dwWindowStatus: 0,
                cxWindowBorders: 0,
                cyWindowBorders: 0,
                atomWindowType: 0,
                wCreatorVersion: 0,
            };

            if winuser::GetWindowInfo(hwnd, &mut window_info) == 0 {
                None
            } else {
                Some(window_info)
            }
        }
    }

    pub fn get_window_rect(hwnd: HWND) -> Option<RECT> {
        let mut rect: RECT = unsafe { mem::zeroed() };
        let success = unsafe { winuser::GetWindowRect(hwnd, &mut rect) };

        if success == 0 {
            None
        }
        else {
            Some(rect)
        }
    }

    pub fn get_foreground_window() -> Option<HWND> {
        unsafe {
            let hwnd = winuser::GetForegroundWindow();

            if hwnd.is_null() {
                None
            }
            else {
                Some(hwnd)
            }
        }
    }

    pub fn set_foreground_window(hwnd: HWND) -> bool {
        unsafe { winuser::SetForegroundWindow(hwnd) != 0 }
    }

    pub fn is_window_visible(hwnd: HWND) -> bool {
        unsafe { winuser::IsWindowVisible(hwnd) != 0 }
    }

    pub fn is_iconic(hwnd: HWND) -> bool {
        unsafe { winuser::IsIconic(hwnd) != 0 }
    }

    pub fn show_window(hwnd: HWND, cmd: i32) -> bool {
        unsafe { winuser::ShowWindow(hwnd, cmd) != 0 }
    }
}

use std::ops::{Add, Sub, Div};
use rsautogui::{mouse, mouse::Button};

#[derive(Debug, Clone)]
pub struct Pos(f64, f64);

impl Pos {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    pub fn scale(&mut self, size: Pos) {
        *&mut self.0 *= size.0;
        *&mut self.1 *= size.1;
    }

    pub fn left_click(&self) {
        mouse::move_to(*&self.0 as u16, *&self.1 as u16);
        mouse::click(Button::Left);
    }
}

impl Div for Pos {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

type Size = Pos;

pub fn get_starting_btn_center_pos(window_pos: Pos, window_size: Size) -> Pos {
    let mut distance_to_lt: Pos = Pos::new(970.0, 915.0);
    let norm_window_size: Pos = Pos::new(1920.0, 1080.0);

    distance_to_lt.scale(window_size / norm_window_size);
    distance_to_lt + window_pos
}

pub fn get_canceling_btn_center_pos(window_pos: Pos, window_size: Size) -> Pos {
    let mut distance_to_lt: Pos = Pos::new(805.0, 675.0);
    let norm_window_size: Pos = Pos::new(1920.0, 1080.0);

    distance_to_lt.scale(window_size / norm_window_size);
    distance_to_lt + window_pos
}