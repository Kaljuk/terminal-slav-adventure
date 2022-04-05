use std::io::{Stdout, Write};

use crossterm::{
  cursor,
  style::{Color, SetBackgroundColor},
  terminal::{Clear, ClearType},
  QueueableCommand,
};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, prev_frame: &Frame, curr_frame: &Frame, force: bool) {
  if force {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(SetBackgroundColor(Color::Black)).unwrap();

    for (x, frame_x) in curr_frame.iter().enumerate() {
      for (y, frame_y) in frame_x.iter().enumerate() {
        if prev_frame[x][y] != *frame_y || force {
          stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
          print!("{}", frame_y)
        }
      }
    }
    stdout.flush().unwrap();
  }
}
