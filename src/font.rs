//! Module contains moves for supported ASCII characters 
//!
//! For now only support A-Z

use microbit::display::nonblocking::BitImage;


/// First ASCII symbol available for printing - 65 is A
const PRINTABLE_START: usize = 65;

/// Last ASCII symbol available for printing - 90 is Z
const PRINTABLE_END: usize = 90;

/// 'Hollow square' movement array
const UNKNOWN_CHARACTER: [[u8;5]; 8] = [[0x1,  0x1,  0x1,  0x1,  0x1],  [0x9, 0x8, 0x8,  0x8, 0x9], [0xD, 0x4, 0x4,  0x4, 0xD], [0xF,  0x2,  0x2,  0x2,  0xF],
                                        [0xDF, 0xD1, 0xD1, 0xD1, 0xDF], [0xDE, 0x8, 0x8, 0x8, 0xDE], [0xD6, 0x4, 0x4, 0x4, 0xD6], [0xD2, 0x2, 0x2, 0x2, 0xD2]
];
   

/// Returns an array that represents movement of the symbol from right to left
///
/// If the requested character isn't supported, returns a 'hollow square' movement array
pub fn get_char_moves(index: u8) -> &'static [[u8; 5]; 8] {
    let index = index as usize;
    if !(PRINTABLE_START..PRINTABLE_START + PRINTABLE_END).contains(&index) {
        return &UNKNOWN_CHARACTER;
    }
    &CHARS_MOVEMENTS[index - PRINTABLE_START]
}

/// Converts move (an array of 5 bytes where each byte represent a row of micro:bit 5*5 display)
/// into BitImage - which is then gets displayed
pub const fn move_to_image(data: [u8; 5]) -> BitImage {
    // Note the data in the pendolino font uses the opposite column numbering
    // system to BitImage.
    const fn row_bits(byte: u8) -> [u8; 5] {[
        ((byte & 1<<4) != 0) as u8,
        ((byte & 1<<3) != 0) as u8,
        ((byte & 1<<2) != 0) as u8,
        ((byte & 1<<1) != 0) as u8,
        ((byte & 1<<0) != 0) as u8,
    ]}
    BitImage::new(&[
        row_bits(data[0]),
        row_bits(data[1]),
        row_bits(data[2]),
        row_bits(data[3]),
        row_bits(data[4]),
    ])
}

/// An array of movements for all supported ASCII characters
const CHARS_MOVEMENTS: [[[u8;5]; 8]; PRINTABLE_END - PRINTABLE_START + 1] = [
    [[0x0, 0x1, 0x1, 0x1, 0x1], [0x1, 0x8, 0x9, 0x8, 0x8], [0x9, 0x4, 0xD, 0x4, 0x4], [0xC, 0x3, 0xF, 0x3, 0x3], [0x6,  0xD8, 0xDE, 0xD8, 0xD8], [0xD2, 0x4,  0xD6, 0x4,  0x4],  [0xD0, 0x2,  0xD2, 0x2,  0x2],  [0xD0, 0xD0, 0xD0, 0xD0, 0xD0]], // Letter A - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x9, 0x8, 0x9], [0xD, 0x4, 0xD, 0x4, 0xD], [0xE, 0x3, 0xE, 0x3, 0xE], [0xD6, 0xD8, 0xD6, 0xD8, 0xD6], [0xD2, 0x4,  0xD2, 0x4,  0xD2], [0xD0, 0x2,  0xD0, 0x2,  0xD0], [0x0,  0xD0, 0x0,  0xD0, 0x0]],  // Letter B - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x8, 0x8, 0x9], [0xD, 0x4, 0x4, 0x4, 0xD], [0xF, 0x2, 0x2, 0x2, 0xF], [0xDE, 0xD0, 0xD0, 0xD0, 0xDE], [0xD6, 0x0,  0x0,  0x0,  0xD6], [0xD2, 0x0,  0x0,  0x0,  0xD2], [0xD0, 0x0,  0x0,  0x0,  0xD0]], // Letter C - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x8, 0x8, 0x9], [0xC, 0x5, 0x4, 0x5, 0xC], [0x6, 0xA, 0x3, 0xA, 0x6], [0xD2, 0xD4, 0xD8, 0xD4, 0xD2], [0xD0, 0x2,  0x4,  0x2,  0xD0], [0x0,  0xD0, 0x2,  0xD0, 0x0],  [0x0,  0x0,  0xD0, 0x0,  0x0]],  // Letter D - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x9, 0x8, 0x9], [0xD, 0x4, 0xD, 0x4, 0xD], [0xF, 0x2, 0xF, 0x2, 0xF], [0xDE, 0xD0, 0xDE, 0xD0, 0xDE], [0xD6, 0x0,  0xD6, 0x0,  0xD6], [0xD2, 0x0,  0xD2, 0x0,  0xD2], [0xD0, 0x0,  0xD0, 0x0,  0xD0]], // Letter E - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x9, 0x8, 0x8], [0xD, 0x4, 0xD, 0x4, 0x4], [0xF, 0x2, 0xF, 0x2, 0x2], [0xDE, 0xD0, 0xDE, 0xD0, 0xD0], [0xD6, 0x0,  0xD6, 0x0,  0x0],  [0xD2, 0x0,  0xD2, 0x0,  0x0],  [0xD0, 0x0,  0xD0, 0x0,  0x0]],  // Letter F - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x8, 0x8, 0x9], [0xD, 0x4, 0x5, 0x4, 0xD], [0xF, 0x2, 0xB, 0x3, 0xF], [0xDE, 0xD0, 0xDC, 0xD8, 0xDE], [0xD6, 0x0,  0x6,  0x4,  0xD6], [0xD2, 0x0,  0xD2, 0x2,  0xD2], [0xD0, 0x0,  0xD0, 0xD0, 0xD0]], // Letter G - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x8, 0x8, 0x8, 0x8], [0x4, 0x4, 0xD, 0x4, 0x4], [0x3, 0x3, 0xF, 0x3, 0x3], [0xD8, 0xD8, 0xDE, 0xD8, 0xD8], [0x4,  0x4,  0xD6, 0x4,  0x4],  [0x2,  0x2,  0xD2, 0x2,  0x2],  [0xD0, 0xD0, 0xD0, 0xD0, 0xD0]], // Letter H - done
    [[0x0, 0x0, 0x0, 0x0, 0x0], [0x1, 0x0, 0x0, 0x0, 0x1], [0x9, 0x1, 0x1, 0x1, 0x9], [0xD, 0x8, 0x8, 0x8, 0xD], [0xE,  0x4,  0x4,  0x4,  0xE],  [0xD6, 0x2,  0x2,  0x2,  0xD6], [0x2,  0xD0, 0xD0, 0xD0, 0x2],  [0xD0, 0x0,  0x0,  0x0,  0xD0]], // Letter I - done
    [[0x0, 0x0, 0x1, 0x1, 0x1], [0x1, 0x0, 0x8, 0x8, 0x9], [0x9, 0x1, 0x5, 0x5, 0xD], [0xD, 0x8, 0xA, 0xA, 0xE], [0xE,  0x4,  0xD4, 0xD4, 0xD6], [0xD6, 0x2,  0x2,  0x2,  0xD2], [0xD2, 0xD0, 0xD0, 0xD0, 0xD0], [0xD0, 0x0,  0x0,  0x0,  0x0]],  // Letter J - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x8, 0x9, 0x8, 0x8], [0x4, 0x5, 0xC, 0x5, 0x4], [0x3, 0xA, 0x6, 0xA, 0x3], [0xD8, 0xD4, 0xD2, 0xD4, 0xD8], [0x4,  0x2,  0xD0, 0x2,  0x4],  [0x2,  0xD0, 0x0,  0xD0, 0x2],  [0xD0, 0x0,  0x0,  0x0,  0xD0]], // Letter K - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x8, 0x8, 0x8, 0x9], [0x4, 0x4, 0x4, 0x4, 0xD], [0x2, 0x2, 0x2, 0x2, 0xF], [0xD0, 0xD0, 0xD0, 0xD0, 0xDE], [0x0,  0x0,  0x0,  0x0,  0xD6], [0x0,  0x0,  0x0,  0x0,  0xD2], [0x0,  0x0,  0x0,  0x0,  0xD0]], // Letter L - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x9, 0x9, 0x8, 0x8], [0x4, 0xD, 0xD, 0x4, 0x4], [0x3, 0xF, 0xF, 0x3, 0x3], [0xD8, 0xDE, 0xDE, 0xD8, 0xD8], [0x4,  0xD6, 0xD6, 0x4,  0x4],  [0x2,  0xD2, 0xD2, 0x2,  0x2],  [0xD0, 0xD0, 0xD0, 0xD0, 0xD0]], // Letter M - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x9, 0x8, 0x8, 0x8], [0x4, 0xC, 0x4, 0x5, 0x4], [0x3, 0x7, 0xB, 0x3, 0x3], [0xD8, 0xDA, 0xDC, 0xD8, 0xD8], [0x4,  0xD4, 0x6,  0x4,  0x4],  [0x2,  0x2,  0xD2, 0x2,  0x2],  [0xD0, 0xD0, 0xD0, 0xD0, 0xD0]], // Letter N - done
    [[0x0, 0x1, 0x1, 0x1, 0x0], [0x1, 0x8, 0x8, 0x8, 0x1], [0x9, 0x4, 0x4, 0x4, 0x9], [0xC, 0x3, 0x3, 0x3, 0xC], [0x6,  0xD8, 0xD8, 0xD8, 0x6],  [0xD2, 0x4,  0x4,  0x4,  0xD2], [0xD0, 0x2,  0x2,  0x2,  0xD0], [0x0,  0xD0, 0xD0, 0xD0, 0x0]],  // Letter O - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x9, 0x8, 0x8], [0xD, 0x4, 0xD, 0x4, 0x4], [0xF, 0x3, 0xF, 0x2, 0x2], [0xDE, 0xD8, 0xDE, 0xD0, 0xD0], [0xD6, 0x4,  0xD6, 0x0,  0x0],  [0xD2, 0x2,  0xD2, 0x0,  0x0],  [0xD0, 0xD0, 0xD0, 0x0,  0x0]],  // Letter P - done
    [[0x0, 0x1, 0x1, 0x1, 0x0], [0x1, 0x8, 0x8, 0x8, 0x1], [0x9, 0x4, 0x4, 0x5, 0x9], [0xC, 0x3, 0x3, 0xB, 0xD], [0x6,  0xD8, 0xD8, 0xDC, 0xE],  [0xD2, 0x4,  0x4,  0x6,  0xD6], [0xD0, 0x2,  0x2,  0xD2, 0xD2], [0x0,  0xD0, 0xD0, 0xD0, 0xD0]], // Letter Q - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x9, 0x8, 0x9, 0x8, 0x8], [0xD, 0x5, 0xD, 0x5, 0x4], [0xE, 0xA, 0xE, 0xA, 0x3], [0xD6, 0xD4, 0xD6, 0xD4, 0xD8], [0xD2, 0x2,  0xD2, 0x2,  0x4],  [0xD0, 0xD0, 0xD0, 0xD0, 0x2],  [0x0,  0x0,  0x0,  0x0,  0xD0]], // Letter R - done
    [[0x0, 0x1, 0x0, 0x0, 0x1], [0x1, 0x8, 0x1, 0x0, 0x9], [0x9, 0x4, 0x9, 0x0, 0xD], [0xD, 0x2, 0xC, 0x1, 0xE], [0xE,  0xD0, 0x6,  0x8,  0xD6], [0xD6, 0x0,  0xD2, 0x4,  0xD2], [0xD2, 0x0,  0xD0, 0x2,  0xD0], [0xD0, 0x0,  0x0,  0xD0, 0x0]],  // Letter S - done
    [[0x0, 0x0, 0x0, 0x0, 0x0], [0x1, 0x0, 0x0, 0x0, 0x0], [0x9, 0x1, 0x1, 0x1, 0x1], [0xD, 0x8, 0x8, 0x8, 0x8], [0xE,  0x4,  0x4,  0x4,  0x4],  [0xD6, 0x2,  0x2,  0x2,  0x2],  [0x2,  0xD0, 0xD0, 0xD0, 0xD0], [0xD0, 0x0,  0x0,  0x0,  0x0]],  // Letter T - done
    [[0x1, 0x1, 0x1, 0x1, 0x0], [0x8, 0x8, 0x8, 0x8, 0x1], [0x4, 0x4, 0x4, 0x4, 0x9], [0x3, 0x3, 0xF, 0x3, 0xC], [0xD8, 0xD8, 0xD8, 0xD8, 0x6],  [0x4,  0x4,  0x4,  0x4,  0xD2], [0x2,  0x2,  0x2,  0x2,  0xD0], [0xD0, 0xD0, 0xD0, 0xD0, 0x0]],  // Letter U - done
    [[0x1, 0x1, 0x1, 0x1, 0x0], [0x8, 0x8, 0x8, 0x8, 0x1], [0x4, 0x4, 0x5, 0x5, 0x8], [0x3, 0x3, 0xA, 0xA, 0x4], [0xD8, 0xD8, 0xD4, 0xD4, 0x2],  [0x4,  0x4,  0x2,  0x2,  0xD0], [0x2,  0x2,  0xD0, 0xD0, 0x0],  [0xD0, 0xD0, 0x0,  0x0,  0x0]],  // Letter V - done
    [[0x1, 0x1, 0x1, 0x1, 0x1], [0x8, 0x8, 0x9, 0x9, 0x8], [0x4, 0x4, 0xD, 0xD, 0x4], [0x3, 0x3, 0xF, 0xF, 0x3], [0xD8, 0xD8, 0xDE, 0xDE, 0xD8], [0x4,  0x4,  0xD6, 0xD6, 0x4],  [0x2,  0x2,  0xD2, 0xD2, 0x2],  [0xD0, 0xD0, 0xD0, 0xD0, 0xD0]], // Letter W - done
    [[0x1, 0x0, 0x0, 0x0, 0x1], [0x8, 0x1, 0x1, 0x1, 0x8], [0x4, 0x9, 0x9, 0x9, 0x4], [0x3, 0xC, 0xC, 0xC, 0x3], [0xD8, 0x6,  0x6,  0x6,  0xD8], [0x4,  0xD2, 0xD2, 0xD2, 0x4],  [0x2,  0xD0, 0xD0, 0xD0, 0x2],  [0xD0, 0x0,  0x0,  0x0,  0xD0]], // Letter X - done
    [[0x1, 0x0, 0x0, 0x0, 0x1], [0x8, 0x1, 0x0, 0x1, 0x8], [0x4, 0x8, 0x1, 0x8, 0x4], [0x3, 0x5, 0x8, 0x4, 0x2], [0xD8, 0xA,  0x4,  0x2,  0xD0], [0x4,  0xD4, 0x2,  0xD0, 0x0],  [0x2,  0x2,  0xD0, 0x0,  0x0],  [0xD0, 0xD0, 0x0,  0x0,  0x0]],  // Letter Y - done
    [[0x1, 0x0, 0x0, 0x0, 0x1], [0x9, 0x0, 0x0, 0x1, 0x9], [0xD, 0x0, 0x1, 0x8, 0xD], [0xF, 0x1, 0x8, 0x4, 0xF], [0xDE, 0x8,  0x4,  0x2,  0xDE], [0xD6, 0x4,  0x2,  0xD0, 0xD6], [0xD2, 0x2,  0xD0, 0x0,  0xD2], [0xD0, 0xD0, 0x0,  0x0,  0xD0]], // Letter Z - done
];
