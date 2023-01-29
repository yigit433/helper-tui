use cursive::{
	Cursive,
    traits::*,
	views::{ViewRef, Dialog, ListView, TextView, Checkbox},
};
use std::{
  env,
  fs,
  io,
};

fn delete_files(path: String) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

       if entry.file_type()?.is_dir() {
           fs::remove_dir_all(&path)?;
       } else {
           fs::remove_file(path)?;
       }
    }

    Ok(())
}

pub fn ui(siv: &mut Cursive)  {
	siv.add_layer(
        Dialog::around(
            ListView::new()
            .child(
                "Trash",
                Checkbox::new().with_name("trash_dir_checkbox"),
            )
            .child(
                "Cache",
                Checkbox::new().with_name("cache_dir_checkbox"),
            )
            .fixed_size((20, 10))
        )
        .title("Clean The Trash")
        .button("Cancel", |s| s.quit())
        .button("Execute", move |btn| {
            let trash_dir_checkbox: ViewRef<Checkbox> = btn.find_name("trash_dir_checkbox").unwrap();
            let cache_dir_checkbox: ViewRef<Checkbox> = btn.find_name("cache_dir_checkbox").unwrap();

            if trash_dir_checkbox.is_checked() {
                delete_files("/home/".to_owned() + env!("USER") + "/.local/share/Trash/").unwrap();
            }
            if cache_dir_checkbox.is_checked() {
                delete_files("/home/".to_owned() + env!("USER") + "/.cache").unwrap();
            }

            if trash_dir_checkbox.is_checked() || cache_dir_checkbox.is_checked() {
                btn.add_layer(
                    Dialog::around(TextView::new("Your computer has been successfully cleaned!"))
                    .button("Quit", |s| s.quit())
                );
            }
        })
    );  
}