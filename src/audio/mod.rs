/**
 * @description Audio controller thread
 */

/// Audio tracks for the into/lauch of the game
const INTRO_TRACKS: [&str; 2] = [
  "assets/audio/intro/intro_a.mp3",
  "assets/audio/intro/intro_b.mp3",
];

/// Main Theme song
pub const MAIN_THEME_SONG: &str = "assets/audio/music/main_sound_track_accordion_trap.mp3";
pub const ACTION_THEME_SONG: &str = "assets/audio/music/main_song_strong_bass.mp3";

pub const fn get_intro_track(random_value: i8) -> &'static str {
  match random_value {
    x if x < 65 => INTRO_TRACKS[0],
    _ => INTRO_TRACKS[1],
  }
}
