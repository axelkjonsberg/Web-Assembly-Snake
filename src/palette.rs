use crate::wasm4;

pub const PALETTE: [u32; 4] = [0x173142, 0xf65e5d, 0xffbc47, 0x40cee3];

#[repr(u16)]
pub enum Color {
    SnakeBody = 0x02,
    SnakeHead = 0x04,
    Fruit = 0x03,
}

pub fn set_palette() {
    unsafe {
        *wasm4::PALETTE = PALETTE;
    }
}

pub fn set_draw_color(color: Color) {
    unsafe { *wasm4::DRAW_COLORS = color as u16 }
}
