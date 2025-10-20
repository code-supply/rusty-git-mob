# rusty-git-mob

A git-mob implementation suitable for Nix users, or anyone else who doesn't
want their git config to be used as a read/write database.

Especially useful in conjunction with Home Manager's `programs.git` module.

## What's a git-mob?

When pair or mob programming, it's useful to mark commits with all of the
people involved. This:

- Gives credit to those authors
- Records who to ask about that code
- Is useful for statistical purposes (this project includes `git-marriages` - see below)

## Installation

Add this flake as an input to your Home Manager flake:

```nix
{
  inputs = {
    git-mob = {
      url = "github:code-supply/rusty-git-mob";
    };
    # etc.
  };
  outputs = { self, home-manager, git-mob, ... }: {}
}
```

Weave the Home Manager module through to your config:

```nix
  homeConfigurations."andrew@some-box" = home-manager.lib.homeManagerConfiguration {
    modules = [
      git-mob.nixosModules.homeManager
    ];
  };
```

Create a coauthors file, by default at `~/.git-coauthors` (or set `GIT_MOB_COAUTHORS`):

```json
{
  "teams": {
    "main": {
      "ab": {
        "name": "Andrew Bruce",
        "email": "me@andrewbruce.net",
        "alternate_emails": [
          "bruciemoose@gmail.com"
        ]
      }
    }
  }
}
```

## Usage

`git mob`

Shows the current mob - it might be empty.

`git mob ab`

Pair up with the person with the 'ab' initials.

`git mob -p # or --pick`

Pick your team from a TUI list.

`git mob --message story-1234 ab`

Pair up with the person with the 'ab' initials and add "story-1234" near the
bottom of future commit messages.

`git solo`

Wipe the mob - you're working on your own for future commits.

`git marriages`

Show statistics about mobs. Currently just an ordered list of mob occurrences.

## Customisation

The following environment variables customise rusty-git-mob's behaviour:

`GIT_MOB_COAUTHORS`

The location of the coauthors file - the read-only database of your coworkers.

`GIT_MOB_LIST`

State file containing the current mob.

`GIT_MOB_TEMPLATE`

The location of the template used for the pre-commit hook.

## See also

git-mob is a popular side-project, and has many implementations!

- [My previous one, in shell](https://github.com/code-supply/git-mob)
- [The Javascript one](https://github.com/rkotze/git-mob)
- [A Rust one](https://github.com/Mubashwer/git-mob)
- [Another Rust one](https://github.com/Frost/git-mob)
- [Yet Another Rust one](https://github.com/jplsek/git-mob-rs)
