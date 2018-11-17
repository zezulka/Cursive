//! observed
use vec::Vec2;
use theme::{Color, Style, ColorStyle};
use std::rc::Rc;

#[derive(Clone)]
///
pub struct ObservedChar {
    grapheme : String,
    style : Rc<Style>
}
///
pub struct ObservedFrame {
    width : usize,
    height : usize,
    buffer : Vec<Option<ObservedChar>>,
    drawn : bool
}

impl ObservedFrame {
    ///
    pub fn new(size : Vec2) -> Self {
        let width : usize = size.y;
        let height : usize = size.y;
        let buffer : Vec<Option<ObservedChar>> = vec![None; width*height];

        ObservedFrame {
            width : width,
            height : height,
            buffer : buffer,
            drawn : false
        }
    }
}