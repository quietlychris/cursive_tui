use cursive::Cursive;
use cursive::views::{Button, Dialog};

struct Data {
    counter: u8,
}

fn main() {
    let mut siv = cursive::default();
    let theme = cursive::theme::Theme::terminal_default();
    siv.set_theme(theme);

    my_list(&mut siv);
    // list_view(&mut siv);

    //siv.add_layer(Dialog::text("This is a bunch of text")
    //    .title("My Title").button("Quit", |siv| show_next(siv)));

    /*     siv.add_layer(Dialog::text("This is a survey!\nPress <Next> when you're ready.")
    .title("Important survey")
    .button("Next", show_next)); */

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

/* fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Did you do the thing?")
        .title("Question 1")
        .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
        .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
        .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
} */

fn my_list(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("To-Do List")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    .child("Activity #1", Button::new("thing", |s| my_activity(s)))
                    .child("Activity #2", Button::new("thing", |s| my_activity(s))),
            ),
    )
}

fn my_activity(s: &mut Cursive) {
    // s.pop_layer();
    s.add_layer(
        Dialog::text("Here's the activity that we're going to do!")
            .title("Name of my activity")
            .button("Do the thing!", |s| s.add_layer(Dialog::info("Try again!")))
            .button("Quit", |s| {
                s.pop_layer();
            }),
    );
}

use cursive::{
    traits::*,
    views::{Checkbox, EditView, LinearLayout, ListView, SelectView, TextArea, TextView},
};

fn list_view(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Please fill out this form")
            .button("Ok", |s| s.quit())
            .content(
                ListView::new()
                    // Each child is a single-line view with a label
                    .child("Name", EditView::new().fixed_width(10))
                    .child("Presentation", TextArea::new().min_height(4))
                    .child(
                        "Receive spam?",
                        Checkbox::new().on_change(|s, checked| {
                            // Enable/Disable the next field depending on this checkbox
                            for name in &["email1", "email2"] {
                                s.call_on_name(name, |view: &mut EditView| {
                                    view.set_enabled(checked)
                                });
                                if checked {
                                    s.focus_name("email1").unwrap();
                                }
                            }
                        }),
                    )
                    .child(
                        "Email",
                        // Each child must have a height of 1 line,
                        // but we can still combine multiple views!
                        LinearLayout::horizontal()
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email1")
                                    .fixed_width(15),
                            )
                            .child(TextView::new("@"))
                            .child(
                                EditView::new()
                                    .disabled()
                                    .with_name("email2")
                                    .fixed_width(10),
                            ),
                    )
                    // Delimiter currently are just a blank line
                    .delimiter()
                    .child(
                        "Age",
                        // Popup-mode SelectView are small enough to fit here
                        SelectView::new()
                            .popup()
                            .item_str("0-18")
                            .item_str("19-30")
                            .item_str("31-40")
                            .item_str("41+"),
                    )
                    .with(|list| {
                        // We can also add children procedurally
                        for i in 0..50 {
                            list.add_child(&format!("Item {i}"), EditView::new());
                        }
                    })
                    .scrollable(),
            ),
    );
}
