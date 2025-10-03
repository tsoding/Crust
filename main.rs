#![no_std]
#![no_main]

use core::ffi::*;
use core::panic::PanicInfo;
use core::mem::zeroed;
use libc::*;

// NOTE: Haven't figured out how to not need this yet
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: What's the best way to implement the panic handler within the Crust spirit
    //   PanicInfo must be passed by reference.
    if let Some(location) = info.location() {
        fprintf!(stderr, c"%.*s:%d: ", location.file().len(), location.file().as_ptr(), location.line());
    }
    fprintf!(stderr, c"panicked");
    if let Some(message) = info.message().as_str() {
        fprintf!(stderr, c": %.*s", message.len(), message.as_ptr());
    }
    fprintf!(stderr, c"\n");
    abort();
}

pub mod raymath {
    use core::ffi::{c_float};
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Vector2 {
        pub x: c_float,
        pub y: c_float,
    }
}

pub mod raylib {
    use core::ffi::{c_int, c_float, c_char};
    use crate::raymath::*;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }

    extern "C" {
        #[link_name="InitWindow"]
        pub fn init_window(width: c_int, height: c_int, title: *const c_char);
        #[link_name="WindowShouldClose"]
        pub fn window_should_close() -> bool;
        #[link_name="ClearBackground"]
        pub fn clear_background(color: Color);
        #[link_name="BeginDrawing"]
        pub fn begin_drawing();
        #[link_name="EndDrawing"]
        pub fn end_drawing();
        #[link_name="CloseWindow"]
        pub fn close_window();
        #[link_name="DrawRectangleV"]
        pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color);
        #[link_name="GetFrameTime"]
        pub fn get_frame_time() -> c_float;
        #[link_name="GetScreenWidth"]
        pub fn get_screen_width() -> c_int;
        #[link_name="GetScreenHeight"]
        pub fn get_screen_height() -> c_int;
    }
}

#[macro_use]
pub mod libc {
    use core::ffi::*;

    pub type FILE = c_void;

    extern "C" {
        pub static stdin: *FILE;
        pub static stdout: *FILE;
        pub static stderr: *FILE;
    }

    #[macro_export]
    macro_rules! fprintf {
        ($stream:expr, $fmt:literal $($args:tt)*) => {{
            use core::ffi::c_int;
            extern "C" {
                #[link_name = "fprintf"]
                pub fn fprintf_raw(stream: *libc::FILE, fmt: *const c_char, ...) -> c_int;
            }
            fprintf_raw($stream, $fmt.as_ptr() $($args)*)
        }};
    }

    #[macro_export]
    macro_rules! printf {
        ($fmt:literal $($args:tt)*) => {{
            use core::ffi::c_int;
            extern "C" {
                #[link_name = "printf"]
                pub fn printf_raw(fmt: *const u8, ...) -> c_int;
            }
            printf_raw($fmt.as_ptr() $($args)*)
        }};
    }

    extern "C" {
        pub fn abort() -> !;
    }

    pub fn realloc_items<T>(ptr: *T, count: usize) -> *T {
        extern "C" {
            #[link_name = "realloc"]
            fn realloc_raw(ptr: *c_void, size: usize) -> *mut c_void;
        }
        realloc_raw(ptr as *c_void, size_of::<T>()*count) as *T
    }

    pub fn free<T>(ptr: *T) {
        extern "C" {
            #[link_name = "free"]
            fn free_raw(ptr: *c_void);
        }
        free_raw(ptr as *c_void);
    }
}

pub mod da { // Dynamic Arrays in Crust
    use crate::libc;
    use core::ptr;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Array<T> {
        pub items: *T,
        pub count: usize,
        pub capacity: usize,
    }

    pub fn da_append<T>(da: *Array<T>, item: T) {
        if (*da).count >= (*da).capacity {
            if (*da).capacity == 0 {
                (*da).capacity = 256;
            } else {
                (*da).capacity *= 2;
            }
            (*da).items = libc::realloc_items((*da).items, (*da).capacity);
        }
        *((*da).items.add((*da).count)) = item;
        (*da).count += 1;
    }

    pub fn da_destroy<T>(da: *Array<T>) {
        libc::free((*da).items);
        (*da).items = ptr::null_mut();
        (*da).count = 0;
        (*da).capacity = 0;
    }
}

#[derive(Copy, Clone)]
pub struct Rect {
    pub position: raymath::Vector2,
    pub velocity: raymath::Vector2,
    pub color: raylib::Color,
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: **u8) -> i32 {
    use core::ffi::c_float;
    use raylib::*;
    use raymath::*;
    use da::*;

    const BACKGROUND: Color = Color {r: 0x18, g: 0x18, b: 0x18, a: 255};
    const RECT_SIZE: Vector2 = Vector2 { x: 100.0, y: 100.0 };

    let rects: Array<Rect> = zeroed();
    da_append(&mut rects, Rect {
        position: Vector2 { x: 0.0, y: 0.0 },
        velocity: Vector2 { x: 100.0, y: 100.0 },
        color: Color {r: 0xFF, g: 0x18, b: 0x18, a: 255},
    });
    da_append(&mut rects, Rect {
        position: Vector2 { x: 300.0, y: 20.0 },
        velocity: Vector2 { x: 100.0, y: 100.0 },
        color: Color {r: 0x18, g: 0xFF, b: 0x18, a: 255},
    });
    da_append(&mut rects, Rect {
        position: Vector2 { x: 20.0, y: 300.0 },
        velocity: Vector2 { x: 100.0, y: 100.0 },
        color: Color {r: 0x18, g: 0x18, b: 0xFF, a: 255},
    });

    init_window(800, 600, c"Hello from Crust".as_ptr());
    while !window_should_close() {
        let dt = get_frame_time();
        let w = get_screen_width() as c_float;
        let h = get_screen_height() as c_float;
        for i in 0..rects.count {
            let rect = rects.items.add(i);
            let nx = (*rect).position.x + (*rect).velocity.x*dt;
            if nx < 0.0 || nx + RECT_SIZE.x >= w {
                (*rect).velocity.x *= -1.0;
            } else {
                (*rect).position.x = nx;
            }
            let ny = (*rect).position.y + (*rect).velocity.y*dt;
            if ny < 0.0 || ny + RECT_SIZE.y >= h {
                (*rect).velocity.y *= -1.0;
            } else {
                (*rect).position.y = ny;
            }
        }

        begin_drawing();
        clear_background(BACKGROUND);
        for i in 0..rects.count {
            let rect = *rects.items.add(i);
            draw_rectangle_v(rect.position, RECT_SIZE, rect.color);
        }
        end_drawing();
    }
    close_window();
    da_destroy(&mut rects);
    0
}
