use core::sync::atomic::{AtomicU8, Ordering};

use crate::util::outb;


const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

const VGA_BUFFER: *mut u16 = 0xB8000 as *mut u16;

static mut WRITER_X: usize = 0;
static mut WRITER_Y: usize = 0;

const WRITER_COLOR: AtomicU8 = AtomicU8::new(
    vga_color(VgaColor::White, VgaColor::Black)
);


#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum VgaColor {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGrey = 7,
	DarkGrey = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	LightMagenta = 13,
	LightBrown = 14,
	White = 15,
}

pub const fn vga_color(foreground: VgaColor, background: VgaColor) -> u8 {
    foreground as u8 | ((background as u8) << 4)
}

pub const fn vga_text(c: char, color: u8) -> u16 {
    c as u16 | ((color as u16) << 8)
}

pub fn set(x: usize, y: usize, ch: u16) {
    if x >= VGA_WIDTH || y >= VGA_HEIGHT {
        return;
    }
    unsafe { *VGA_BUFFER.add((y * VGA_WIDTH as usize) + x) = ch; }
}

pub fn clear_screen() {
    let ch = vga_text(' ', WRITER_COLOR.load(Ordering::Relaxed));
    for x in 0..VGA_WIDTH {
        for y in 0..VGA_HEIGHT {
            set(x, y, ch);
        }
    }
}

pub fn scan_up() { unsafe {
    let ch = vga_text(' ', WRITER_COLOR.load(Ordering::Relaxed));
    for x in 0..VGA_WIDTH {
        for y in 1..VGA_HEIGHT-1 {
            *VGA_BUFFER.add((y - 1 * VGA_WIDTH) + x) =
            *VGA_BUFFER.add((y * VGA_WIDTH) + x)
        }
    }
    for x in 0..VGA_WIDTH {
        let y = VGA_HEIGHT-1;
        *VGA_BUFFER.add((y * VGA_WIDTH) + x) = ch;
    }
}}


pub fn set_color(color: u8) {
    WRITER_COLOR.store(color, Ordering::Relaxed);
}
pub fn set_x(x: usize) {
    if x >= VGA_WIDTH {
        return;
    }
    unsafe {
        WRITER_X = x;
    }
}
pub fn set_y(y: usize) {
    if y >= VGA_HEIGHT {
        return;
    }
    unsafe {
        WRITER_Y = y;
    }
}

// now for the cool parts

pub fn write_char(ch: char) {

    if ch == '\n' {
        unsafe {
            WRITER_X = 0;
            WRITER_Y += 1;
        }
        return;
    }
    // check WRITER_(X,Y) bounds

    let x = unsafe {
        if WRITER_X >= VGA_WIDTH {
            WRITER_X = 0;
            WRITER_Y += 1;
        }
        WRITER_X
    };
    let y = unsafe {
        if WRITER_Y >= VGA_HEIGHT {
            scan_up();
            WRITER_Y = VGA_HEIGHT - 1;
        }
        WRITER_Y
    };

    set(x, y, vga_text(ch, WRITER_COLOR.load(Ordering::Relaxed)));
    unsafe { WRITER_X += 1 };

}

pub fn write_str(string: &str) {
    for ch in string.chars() {
        write_char(ch);
    }
}

pub fn disable_cursor() {
    outb(0x3D4, 0x0A);
	outb(0x3D5, 0x20);
}
