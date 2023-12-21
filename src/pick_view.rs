use crate::git_mob::output;
use crate::git_mob::trailers;
use crate::git_mob::Coauthors;
use crate::git_mob::Output;
use cursive::views::OnEventView;
use cursive::views::ScrollView;
use cursive::views::{Checkbox, Dialog};
use cursive::{event::Key, views::ListView};
use std::collections::HashSet;

pub(crate) fn render<F>(coauthors: Coauthors, mob: &HashSet<String>, write: F)
where
    F: Fn(Output) -> Result<(), Box<dyn std::error::Error>> + 'static,
{
    let mut siv = cursive::default();

    siv.set_user_data(mob.to_owned());

    siv.add_layer(
        OnEventView::new(dialog(scroll_view(&coauthors, mob), coauthors, write))
            .on_event('q', |s| s.quit())
            .on_event(Key::Esc, |s| s.quit()),
    );

    siv.run()
}

fn dialog<F>(
    view: ScrollView<ListView>,
    coauthors: std::collections::HashMap<String, crate::git_mob::Coauthor>,
    write: F,
) -> Dialog
where
    F: Fn(Output) -> Result<(), Box<dyn std::error::Error>> + 'static,
{
    Dialog::around(view)
        .title("Mob up with")
        .button("Cancel", |s| s.quit())
        .button("OK", move |s| {
            s.with_user_data(|mob: &mut HashSet<String>| {
                write(output(&trailers(&coauthors, mob), mob))
            });
            s.quit()
        })
}

fn scroll_view(
    coauthors: &std::collections::HashMap<String, crate::git_mob::Coauthor>,
    mob: &HashSet<String>,
) -> ScrollView<ListView> {
    ScrollView::new(
        coauthors
            .clone()
            .into_iter()
            .fold(ListView::new(), |list_view, (initials, coauthor)| {
                list_view.child(&coauthor.name, checkbox(mob, initials))
            }),
    )
}

fn checkbox(mob: &HashSet<String>, initials: String) -> Checkbox {
    Checkbox::new()
        .with_checked(mob.contains(&initials))
        .on_change(move |s, checked| {
            s.with_user_data(|mob: &mut HashSet<String>| {
                if checked {
                    mob.insert(initials.clone());
                } else {
                    mob.remove(&initials);
                }
            });
        })
}
