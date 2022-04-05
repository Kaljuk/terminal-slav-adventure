use crossterm::{
  cursor,
  event::{self, Event, KeyCode},
  terminal::{self},
  ExecutableCommand,
};
use rodio::{OutputStream, Sink};
use std::{
  error::Error,
  fs::File,
  io::{self, BufReader},
  sync::mpsc::channel,
  thread::{self, sleep, spawn},
  time::Duration,
};
use terminal_slav_adventure::{
  audio::{get_intro_track, ACTION_THEME_SONG, MAIN_THEME_SONG},
  frame::{self, new_frame},
  render,
};

fn main() -> Result<(), Box<dyn Error>> {
  // TODO: Add audio into a separate thread and activate it in this one
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();
  sink.set_volume(0.5);

  let mut exit_game = false;

  let intro_track_path = get_intro_track(rand::random::<i8>());
  let intro_track_file = BufReader::new(File::open(intro_track_path)?);
  let mut source = rodio::Decoder::new(intro_track_file)?;
  sink.append(source);

  let audio_file = BufReader::new(File::open(MAIN_THEME_SONG)?);
  source = rodio::Decoder::new(audio_file)?;
  sink.append(source);

  // Gameview - Enter separate terminal and hide cursor
  let mut stdout = io::stdout();
  terminal::enable_raw_mode()?;
  stdout.execute(terminal::EnterAlternateScreen)?;
  stdout.execute(cursor::Hide)?;

  // Render loop thread
  let (render_tx, render_rx) = channel();
  let render_handle = spawn(move || {
    let mut last_frame = frame::new_frame();
    let mut stdout = io::stdout();

    render::render(&mut stdout, &last_frame, &last_frame, true);

    while let Ok(current_frame) = render_rx.recv() {
      render::render(&mut stdout, &last_frame, &current_frame, false);
      last_frame = current_frame;

      sleep(Duration::from_millis(50));
    }

    //loop {
    //  let curr_frame = match render_rx.recv() {
    //    Ok(x) => x,
    //    Err(_) => break,
    //  };
    //  render::render(&mut stdout, &last_frame, &curr_frame, false);
    //  last_frame = curr_frame;
    //}
  });

  'gameloop: loop {
    // Init frame
    let curr_frame = new_frame();
    // Input
    while event::poll(Duration::default())? {
      if let Event::Key(key_event) = event::read()? {
        match key_event.code {
          // End game
          KeyCode::Esc | KeyCode::Char('q') => {
            exit_game = true;
            break 'gameloop;
          }
          KeyCode::Char('+') => sink.set_volume(sink.volume() + 0.1),
          KeyCode::Char('-') => sink.set_volume(sink.volume() - 0.1),
          KeyCode::Enter => {
            let action_audio_file = BufReader::new(File::open(ACTION_THEME_SONG)?);
            source = rodio::Decoder::new(action_audio_file)?;
            sink.append(source);
          }
          _ => {}
        }
      };
    }
    // Draw and render
    let _ = render_tx.send(curr_frame).unwrap();
    thread::sleep(Duration::from_millis(2));
  }

  if exit_game {
    sink.stop();
  } else {
    // Wait until the music ends before continuing
    sink.sleep_until_end();
  }

  // Cleanup
  drop(render_tx);
  render_handle.join().unwrap();
  // Show cursor and exit gamescreen
  stdout.execute(cursor::Show)?;
  stdout.execute(terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;
  Ok(())
}
