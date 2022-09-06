//! Module contains code for Simple Text Scroller 


use microbit::display::nonblocking::BitImage;
use crate::font;
use tiny_led_matrix::Render;
use heapless::Vec;


/// The state of an animation
pub trait Animate {
    /// Subimage should implement Render trait to ensure it could be displayed by micro:bit
    type Subimage: Render;

    /// Creates instance that would hold information for scrollable text
    fn new(message: &str) -> Self;

    /// Returns true when all images were shown
    fn is_finished(&mut self) -> bool;

    /// Reset the animation to the beginning
    fn reset(&mut self);

    /// Returns next animation image
    fn next(&mut self) -> &Self::Subimage;
}


/// ScrollMessage is the struct to enable scrollable text
pub struct ScrollMessage<const N: usize> {
    states:     Vec<BitImage, N>,
    tick:       usize,
    scroll_len: usize
}


impl<const N: usize> Animate for ScrollMessage<N> {
    type Subimage = BitImage;

    fn new(message: &str) -> Self {
        let mut states: Vec<BitImage, N> = Vec::new();

        for byte in message.bytes() {
            let moves = font::get_char_moves(byte);
            for mv in moves {
                states.push(font::move_to_image(*mv)).unwrap();
            }
        }
        ScrollMessage {
            states,
            tick: 0,
            scroll_len: message.len() * 8

        }
    }

    fn next(&mut self) -> &Self::Subimage {
        let t = self.tick;
        self.tick += 1;
        
        &self.states.get(t).unwrap()
    }

    fn is_finished(&mut self) -> bool {
        self.tick == self.scroll_len
    }

    fn reset(&mut self) {
        self.tick = 0;
    }
}
