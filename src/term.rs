use std::{sync::mpsc::{Receiver, self}, thread, io::stdin};
use termion::{input::{TermRead}, event::{Event, MouseEvent, Key} };

use crate::vector::Vector;


pub type OffthreadStdin = Receiver<Event>;
pub fn offthread_stdin() -> OffthreadStdin {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for e in stdin.events() {
            tx.send(e.unwrap()).unwrap();
        }
    });
    rx
}

pub fn collect_events(stdin: &OffthreadStdin) -> Vec<Event> {
  let mut events = vec![];
  loop {
      if let Ok(event) = stdin.try_recv() {
          events.push(event);
      } else {
          return events;
      }
  }
}

pub fn key_press (events: &Vec<Event>, key: &Key) -> bool {
  for event in events {
      if let Event::Key(found_key) = event {
          if found_key == key {
              return true;
          }
      }
  }
  false
}

pub fn pressed_between(events: &Vec<Event>, start: Vector, end: Vector) -> bool {
  for event in events {
      if let Event::Mouse(MouseEvent::Press(_, x, y)) = event {
          let x = *x as i32;
          let y = *y as i32;
          if x < start.x {
              return false;
          } else if x > end.x {
              return false;
          } else if y < start.y {
              return false;
          } else if y > end.y {
              return false;
          } else {
              return true;
          }
      }
  }
  false
}

#[macro_export]
macro_rules! write_at {
  ($stdout:ident, $str:expr) => {
      write!($stdout, "{}", $str).unwrap()
  };
  ($stdout:ident, $vec:expr, $str:expr) => {
      write_at!($stdout, $vec.x, $vec.y, $str)
  };

  ($stdout:ident, $x:expr, $y:expr, $str:expr) => {
      write!($stdout, "{}{}", termion::cursor::Goto($x as u16, $y as u16), $str).unwrap();
  };
}

#[macro_export]
macro_rules! write_many_at {
  ($stdout:ident, $lines:expr) => {
      $lines.for_each(|(pos, text)| {
          write_at!($stdout, pos, text);
      })
  };

  ($stdout:ident, $offset:expr, $lines:expr) => {
      write_many_at!(
          $stdout,
          $lines
              .map(|(pos, text)| (pos + $offset, text))
      )
  };
}
