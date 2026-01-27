{ packages }:

{
  config,
  lib,
  pkgs,
  ...
}:

with lib;

let
  cfg = config.programs.rusty-git-mob;
  gitMob = packages.${pkgs.stdenv.hostPlatform.system}.default;
in
{
  options.programs.rusty-git-mob = {
    enable = mkEnableOption "rusty-git-mob";
  };

  config = mkIf cfg.enable {
    home.packages = [ gitMob ];
    programs.git.hooks.prepare-commit-msg = "${gitMob}/bin/git-mob-prepare-commit-msg";
    programs.git.settings.commit.template = "~/.gitmessage.txt";
  };
}
