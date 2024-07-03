use std::{process, thread, time::Duration};
use winapi::um::winuser::{SW_SHOW, WS_CAPTION};
use zzz_uid_predator::{self, Pos, win32};
use rand::Rng;

fn main() {
    println!("Checking whether UAC is granted...");
    if !win32::is_user_an_admin() {
        println!("This program must have UAC permission!");
        process::exit(1);
    }

    println!("Trying to get ZZZ window...");
    
    let hwnd = win32::find_window(None, "绝区零").expect("Cannot find the ZZZ window");
    println!("Success!");

    println!("Trying to show the window...");

    if !win32::show_window(hwnd, SW_SHOW) {
        panic!("Cannot set ZZZ window to be shown");
    }

    println!("Starting to enter...");
    println!("If you want to cancel, just hide that window.");

    while !win32::is_iconic(hwnd) {
        if !win32::set_foreground_window(hwnd) {
            panic!("Cannot set ZZZ window to be foreground");
        }

        let rect = win32::get_window_rect(hwnd).expect("Cannot get the position and size of this ZZZ window");
        println!("The position and the size of the ZZZ window are:");
        println!("  Left: {}, Top: {}, Right: {}, Bottom: {}", rect.left, rect.top, rect.right, rect.bottom);
        
        let mut size = Pos::new((rect.right - rect.left) as f64, (rect.bottom - rect.top) as f64);
        let mut pos = Pos::new(rect.left as f64, rect.top as f64);

        let window_info = win32::get_window_info(hwnd).expect("Cannot get information of the window");

        if window_info.dwStyle != WS_CAPTION {
            // let caption_size = Pos::new(window_info.cxWindowBorders as f64, window_info.cyWindowBorders as f64);
            let caption_size = Pos::new(8.0, 32.0);

            size = size - caption_size.clone();
            pos = pos - caption_size.clone();
        }

        zzz_uid_predator
            ::get_starting_btn_center_pos(pos.clone(), size.clone())
            .left_click();

        thread::sleep(Duration::from_secs(1));

        zzz_uid_predator
            ::get_canceling_btn_center_pos(pos.clone(), size.clone())
            .left_click();

        println!("Waitting for 12±12 secs...");

        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_secs(
            rng.gen_range(12..=24) - 12
            + 12
        ));
    }
    
    println!("Attempts have been stopped due to window is not focused now!");
}
