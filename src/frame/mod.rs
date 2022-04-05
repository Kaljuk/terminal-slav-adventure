use crate::{RESOLUTION_X, RESOLUTION_Y};

// TODO: Move Frame into rendering module - its a rendering subtask, so no need to be a separate lib

pub type Frame = Vec<Vec<&'static str>>;

// Stack the x axis with stacked y axis
pub fn new_frame() -> Frame {
  let mut frame_x = Vec::with_capacity(RESOLUTION_X);
  for _ in 0..RESOLUTION_X {
    let mut frame_y = Vec::with_capacity(RESOLUTION_Y);
    for _ in 0..RESOLUTION_Y {
      frame_y.push(" ");
    }
    frame_x.push(frame_y);
  }
  frame_x
}

pub trait Drawable {
  fn draw(&self, frame: &mut Frame);
}
