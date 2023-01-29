use cursive::{
    Cursive,
    align::HAlign,
    traits::*,
    views::{Dialog, SelectView},
};
mod tools;

fn main() {
    let mut menu = SelectView::new()
    .h_align(HAlign::Center)
    .autojump();    

    menu.add_item("Clean the Trash", "cleanthetrash");
    menu.set_on_submit(|siv: &mut Cursive, pname: &str| {
        siv.pop_layer();

        if pname == "cleanthetrash" {
            tools::clean_the_trash::ui(siv);
        }
    });

    let mut siv = cursive::default();
    
    siv.add_layer(
        Dialog::around(menu.scrollable().fixed_size((20, 10)))
        .title("Helper CLI")
        .button("Quit", |s| s.quit())
    );

    siv.run();
}
