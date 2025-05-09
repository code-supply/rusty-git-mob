use cursive::views::OnEventView;
use cursive::views::ScrollView;
use cursive::views::{Checkbox, Dialog};
use cursive::{event::Key, views::ListView};

use crate::config;
use crate::config::CurrentMobInitials;
use crate::config::MobData;
use crate::config::Org;
use crate::config::Team;
use crate::git_mob::output;
use crate::git_mob::Output;
use crate::output::trailers;
use crate::output::MainResult;

pub fn run<F>(org: Org, mob: &MobData, write: F)
where
    F: Fn(Output) -> MainResult + 'static,
{
    let coauthors = config::whole_org_as_team(&org);
    let mut siv = cursive::default();

    siv.set_user_data(mob.to_owned().current_mob_initials.to_owned());

    siv.add_layer(
        OnEventView::new(dialog(
            scroll_view(&coauthors, &mob.current_mob_initials),
            coauthors,
            write,
        ))
        .on_event('q', |s| s.quit())
        .on_event(Key::Esc, |s| s.quit()),
    );

    siv.run()
}

fn dialog<F>(view: ScrollView<ListView>, coauthors: Team, write: F) -> Dialog
where
    F: Fn(Output) -> MainResult + 'static,
{
    Dialog::around(view)
        .title("Mob up with")
        .button("OK", move |s| {
            s.with_user_data(|mob: &mut CurrentMobInitials| {
                let ts = trailers(&coauthors, mob);
                write(output(&None, &ts, mob))
            });
            s.quit()
        })
}

fn scroll_view(coauthors: &Team, mob: &CurrentMobInitials) -> ScrollView<ListView> {
    ScrollView::new(
        coauthors
            .iter()
            .fold(ListView::new(), |list_view, (initials, coauthor)| {
                list_view.child(&coauthor.name, checkbox(mob, initials.to_owned()))
            }),
    )
}

fn checkbox(mob: &CurrentMobInitials, initials: String) -> Checkbox {
    Checkbox::new()
        .with_checked(mob.contains(&initials))
        .on_change(move |s, checked| {
            s.with_user_data(|mob: &mut CurrentMobInitials| {
                if checked {
                    mob.insert(initials.clone());
                } else {
                    mob.remove(&initials);
                }
            });
        })
}
