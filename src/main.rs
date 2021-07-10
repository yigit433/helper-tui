use cursive::{
  Cursive,
  align::HAlign,
  event::EventResult,
  traits::*,
  views::{Dialog, OnEventView, SelectView, TextView}
};
use std::{
  env,
  fs,
  process::{
    Command
  },
  str::{
    from_utf8
  }
};

fn main() {
  let mut select = SelectView::new()
  .h_align(HAlign::Center)
  .autojump();  

  select.add_item("Clean the Trash", "clean_the_trash");
  select.add_item("Connect wifi", "connect_wifi");

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
  
  siv.load_toml(include_str!("theme-config.toml")).unwrap();

  siv.add_layer(
    Dialog::around(select.scrollable().fixed_size((20, 10)))
    .title("Helper CLI")
    .button("Quit", |s| s.quit())
  );

  siv.run();
}

fn start_processing(siv: &mut Cursive, process_name: &str) {
  siv.pop_layer();
  
  if process_name == "clean_the_trash" {
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
  } else if process_name == "connect_wifi" {
    // nmcli --get-values ssid,security,signal device wifi
    let mut output = Command::new("nmcli");
    output.args(&["--get-values=ssid,security,bars", "device", "wifi", "list"]);
  
    let parsed = from_utf8(&output.output().expect("fail").stdout).unwrap().to_string();

    if parsed == "fail" {
      siv.add_layer(
        Dialog::around(TextView::new("An error has been encountered! nmcli not found!"))
        .button("Quit", |s| s.quit())
      );  
    } else {
      let networks = parsed.split("\n").filter(|e| !e.is_empty());
    
      let mut select = SelectView::new()
      .h_align(HAlign::Center)
      .autojump();
      
      select.add_all_str(networks);
      select.set_on_submit(|siv: &mut Cursive, _connection_name: &str| {
        siv.pop_layer();
        let parsed_connection_name = &_connection_name.split(":").into_iter().collect::<Vec<&str>>();
         
        siv.add_layer(
          Dialog::around(TextView::new(format!("{:?}", parsed_connection_name)))
          .button("Quit", |s| s.quit())
        );        
      });

      let select = OnEventView::new(select)
      .on_pre_event_inner('k', |s, _| {
        let cb = s.select_up(1);
        Some(EventResult::Consumed(Some(cb)))
      })
      .on_pre_event_inner('j', |s, _| {
        let cb = s.select_down(1);
        Some(EventResult::Consumed(Some(cb)))
      });

      siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((40, 100)))
        .title("Select network")
        .button("Quit", |s| s.quit())
      );
    }
  }
}
