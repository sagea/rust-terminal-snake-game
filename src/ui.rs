use termion::event::Event;

use crate::{utils::char_len, vector::Vector, term::pressed_between, v};


pub fn button(position: Vector, text: String, events: &Vec<Event>, on_click: impl FnOnce(), on_render: impl FnOnce(Vec<(Vector, String)>)) {
  let text_len = char_len(&text);
  let pos_offset = v!(text_len / 2, 0);
  let pos_start = position - pos_offset;
  let pos_end = position + pos_offset;
  let deets = vec![(pos_start, text)];
  if pressed_between(events, pos_start, pos_end) {
      on_click();
  }
  on_render(deets);
}

pub fn text(position: Vector, text: String, on_render: impl FnOnce(Vec<(Vector, String)>)) {
  let text_len = char_len(&text);
  let pos_offset = v!(text_len / 2, 0);
  let pos_start = position - pos_offset;
  let deets = vec![(pos_start, text)];
  on_render(deets);
}
