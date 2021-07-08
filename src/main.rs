extern crate cursive;
use cursive::{
  Cursive,
  align::HAlign,
  event::EventResult,
  traits::*,
  views::{Dialog, OnEventView, SelectView, TextView}
};
use std::{env,fs};

fn main() {
  let mut select = SelectView::new()
  .h_align(HAlign::Center)
  .autojump();  

  select.add_item("Clean computer", "clean_computer");

  select.set_on_submit(start_processing);

  let select = OnEventView::new(select)
  .on_pre_event_inner('k', |s, _| {
    let cb = s.select_up(1);
    Some(EventResult::Consumed(Some(cb)))
  })
  .on_pre_event_inner('j', |s, _| {
    let cb = s.select_down(1);
    Some(EventResult::Consumed(Some(cb)))
  });

  let mut siv = cursive::default();

  siv.add_layer(
    Dialog::around(select.scrollable().fixed_size((20, 10)))
    .title("Helper CLI")
    .button("Quit", |s| s.quit())
  );

  siv.run();
}

fn start_processing(siv: &mut Cursive, process_name: &str) {
  siv.pop_layer();
  
  if process_name == "clean_computer" {
    let trash_directory = "/home/".to_owned() + env!("USER") + "/.local/share/Trash/";
    let cache_directory = "/home/".to_owned() + env!("USER") + "/.cache";

    if let Err(_err) = fs::remove_dir_all(&trash_directory) {
      siv.add_layer(
        Dialog::around(TextView::new("A trash directory cannot be removed."))
        .button("Quit", |s| s.quit())
      );
    } else if let Err(_err) = fs::remove_dir_all(&cache_directory) {
      siv.add_layer(
        Dialog::around(TextView::new("A cache directory cannot be removed."))
        .button("Quit", |s| s.quit())
      );
    } else {
      if let Err(_err) = fs::create_dir(&trash_directory) {
        siv.add_layer(
          Dialog::around(TextView::new("Could not create Trash directory!"))
          .button("Quit", |s| s.quit())
        );
      } else {
        siv.add_layer(
          Dialog::around(TextView::new("Your computer has been successfully cleaned!"))
          .button("Quit", |s| s.quit())
        );
      }
    } 
  }
}
