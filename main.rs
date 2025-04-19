#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::mem::zeroed;

#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod raymath {
    use core::ffi::{c_float};
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Vector2 {
        pub x: c_float,
        pub y: c_float,
    }
}

mod raylib {
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

    #[link(name="libraylib.a", kind="static")]
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
mod libc {
    use core::ffi::{c_void};

    #[macro_export]
    macro_rules! panic {
        () => {
            {
                use core::ffi::c_int;
                use core::panic::Location;
                extern "C" {
                    #[link_name = "exit"]
                    pub fn exit_raw(code: c_int) -> !;

                    #[link_name = "printf"]
                    pub fn printf_raw(fmt: *const u8, ...) -> c_int;
                }
                // Get the location of the function that called panic
                let loc = Location::caller();
                let file = loc.file();
                let line = loc.line();
                let col  = loc.column();
                printf_raw(b"%.*s:%u:%u: ERROR\n\0".as_ptr(), file.len(), file.as_ptr(), line, col);
                exit_raw(69)
            }
        };
        ($fmt:literal) => {
            {
                use core::ffi::c_int;
                use core::panic::Location;
                extern "C" {
                    #[link_name = "exit"]
                    pub fn exit_raw(code: c_int) -> !;

                    #[link_name = "printf"]
                    pub fn printf_raw(fmt: *const u8, ...) -> c_int;
                }
                // Get the location of the function that called panic
                let loc = Location::caller();
                let file = loc.file();
                let line = loc.line();
                let col  = loc.column();
                printf_raw(b"%.*s:%u:%u: ERROR: %.*s\n\0".as_ptr(), file.len(), file.as_ptr(), line, col, $fmt.len(), $fmt.as_ptr());
                exit_raw(69)
            }
        };
    }

    #[macro_export]
    macro_rules! printf {
        ($fmt:literal $($args:tt)*) => {
            use core::ffi::c_int;
            extern "C" {
                #[link_name = "printf"]
                pub fn printf_raw(fmt: *const u8, ...) -> c_int;
            }
            printf_raw($fmt.as_ptr() $($args)*)
        };
    }

    pub unsafe fn realloc<T>(ptr: *mut T, count: usize) -> *mut T {
        extern "C" {
            #[link_name = "realloc"]
            fn realloc_raw(ptr: *mut c_void, size: usize) -> *mut c_void;
        }
        realloc_raw(ptr as *mut c_void, size_of::<T>()*count) as *mut T
    }

    pub unsafe fn free<T>(ptr: *mut T) {
        extern "C" {
            #[link_name = "free"]
            fn free_raw(ptr: *mut c_void);
        }
        free_raw(ptr as *mut c_void);
    }
}

mod ds { // Data Structures
    use crate::libc;
    use core::ptr;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Array<T> {
        pub items: *mut T,
        pub count: usize,
        pub capacity: usize,
    }

    // This makes the panic::Location track the location of the caller rather than this function
    // if we panic
    #[track_caller]
    pub unsafe fn array_push<T>(xs: *mut Array<T>, item: T) {
        if (*xs).count >= (*xs).capacity {
            if (*xs).capacity == 0 {
                (*xs).capacity = 256;
            } else {
                (*xs).capacity *= 2;
            }
            (*xs).items = libc::realloc((*xs).items, (*xs).capacity);
            if (*xs).items.is_null() {
                panic!("Failed to allocate memory for array.");
            }
        }
        *((*xs).items.add((*xs).count)) = item;
        (*xs).count += 1;
    }

    // This makes the panic::Location track the location of the caller rather than this function
    // if we panic
    #[track_caller]
    pub unsafe fn array_destroy<T>(xs: *mut Array<T>) {
        if (*xs).items.is_null() {
            panic!("Tried to free empty array.");
        }
        libc::free((*xs).items);
        (*xs).items = ptr::null_mut();
        (*xs).count = 0;
        (*xs).capacity = 0;
    }
}

#[derive(Copy, Clone)]
struct Rect {
    position: raymath::Vector2,
    velocity: raymath::Vector2,
    color: raylib::Color,
}

#[no_mangle]
unsafe extern "C" fn main(_argc: i32, _argv: *mut *mut u8) -> i32 {
    use core::ffi::c_float;
    use raylib::*;
    use raymath::*;
    use ds::*;

    const BACKGROUND: Color = Color {r: 0x18, g: 0x18, b: 0x18, a: 255};
    const RECT_SIZE: Vector2 = Vector2 { x: 100.0, y: 100.0 };

    let mut rects: Array<Rect> = zeroed();
    array_push(&mut rects, Rect { 
        position: Vector2 { x: 0.0, y: 0.0 },
        velocity: Vector2 { x: 100.0, y: 100.0 },
        color: Color {r: 0xFF, g: 0x18, b: 0x18, a: 255},
    });
    array_push(&mut rects, Rect { 
        position: Vector2 { x: 300.0, y: 20.0 },
        velocity: Vector2 { x: 100.0, y: 100.0 },
        color: Color {r: 0x18, g: 0xFF, b: 0x18, a: 255},
    });
    array_push(&mut rects, Rect { 
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
    array_destroy(&mut rects);
    // TODO: Remove this later, this is just for testing the panic
    array_destroy(&mut rects);
    0
}
