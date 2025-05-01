use cursive::Cursive;
use cursive::event::Key;
use cursive::views::{Button, Dialog, ResizedView, TextContent};
use cursive::{
    traits::*,
    views::{Checkbox, EditView, LinearLayout, ListView, SelectView, TextArea, TextView},
};

struct Data {
    counter: i32,
}

fn main() {
    let mut siv = cursive::default();
    let theme = cursive::theme::Theme::terminal_default();
    siv.set_theme(theme);
    let my_data = Data { counter: 0 };
    siv.set_user_data(my_data);
    siv.add_global_callback(Key::Esc, |s| s.quit());

    my_list(&mut siv);
    // list_view(&mut siv);

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Did you do the thing?")
            .title("Question 1")
            .button("Yes!", |s| ()) //< Do something interesting here...
            .button("No!", |s| ()) //< And here as well...
            .button("Uh?", |s| ()),
    ); //< And finally here too.
}

use cursive::view::SizeConstraint;
fn my_list(siv: &mut Cursive) {
    let dialog = Dialog::new()
        .title("To-Do List")
        .button("Ok", |s| s.quit())
        .button("Refresh", |s| s.noop())
        .content(
            ListView::new()
                .child("Activity #1", Button::new("thing", |s| my_activity(s)))
                .child("Activity #2", Button::new("thing", |s| my_activity(s)))
                .child("Counter", Button::new("yis", |s| counter(s))),
        );
    let rs = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, dialog);

    siv.add_layer(rs);
}

fn counter(siv: &mut Cursive) {
    let content = TextContent::new("content");
    let c2 = content.clone();
    let view = TextView::new_with_content(content.clone());
    let dialog = Dialog::text("Let's get some user data")
        .content(view)
        .title("Counter")
        .button("Up!", move |s| {
            s.with_user_data(|data: &mut Data| {
                data.counter += 1;
                c2.set_content(data.counter.to_string());
            });
        })
        .button("Down!", move |s| {
            s.with_user_data(|data: &mut Data| {
                data.counter -= 1;
                content.set_content(data.counter.to_string());
            });
        })
        .button("Print!", |s| {
            println!("Thing!");
        })
        .button("Quit", |s| {
            s.pop_layer();
        });
    let rs = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, dialog);
    siv.add_layer(rs);
}

fn my_activity(s: &mut Cursive) {
    // s.pop_layer();

    let dialog = Dialog::text("Here's the activity that we're going to do!")
        .title("Name of my activity")
        .button("Do the thing!", |s| {
            s.add_layer(Dialog::info("Did the thing!"))
        })
        .button("Quit", |s| {
            s.pop_layer();
        });
    let rs = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, dialog);

    s.add_layer(rs);
}
