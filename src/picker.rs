use crate::git_mob::trailers;
use crate::git_mob::Coauthors;
use crate::git_mob::MainResult;
use crate::git_mob::Mob;
use crate::git_mob::Output;
use cursive::views::OnEventView;
use cursive::views::ScrollView;
use cursive::views::{Checkbox, Dialog};
use cursive::{event::Key, views::ListView};

pub(crate) fn run<F>(coauthors: Coauthors, mob: &Mob, write: F)
where
    F: Fn(Output) -> MainResult + 'static,
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
    F: Fn(Output) -> MainResult + 'static,
{
    Dialog::around(view)
        .title("Mob up with")
        .button("OK", move |s| {
            s.with_user_data(|mob: &mut Mob| {
                let ts = trailers(&coauthors, mob);
                write(Output {
                    message: ts.clone(),
                    template: ts,
                    mob: mob.clone(),
                })
            });
            s.quit()
        })
}

fn scroll_view(
    coauthors: &std::collections::HashMap<String, crate::git_mob::Coauthor>,
    mob: &Mob,
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

fn checkbox(mob: &Mob, initials: String) -> Checkbox {
    Checkbox::new()
        .with_checked(mob.contains(&initials))
        .on_change(move |s, checked| {
            s.with_user_data(|mob: &mut Mob| {
                if checked {
                    mob.insert(initials.clone());
                } else {
                    mob.remove(&initials);
                }
            });
        })
}
