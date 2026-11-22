# scrappy

A small, crappy text editor. Maybe it'll grow up to be a real boy.

# Project state

Currently, scrappy can manipulate the clipboard, read and write files, and has
an about screen.

This project uses a variation of Semantic Versioning. Versions below 1.0 do not
track breaking changes. There will be a system for automatically identifying
breaking changes before this project hits 1.0, but it isn't here yet.

Development happens on Forgejo at https://code.i.xenrelay.com/minifig/scrappy:
check its Issues, Milestones, and Pull Requests for known issues, future
features, and work in progress. CI runs there (`.forgejo/workflows/`). The
GitHub repository is a push mirror that only publishes releases.

# Releasing

Set `version` in `Cargo.toml`, commit, then tag `vX.Y.Z` and push the tag to
Forgejo. Forgejo builds a Linux binary for its own release; the tag reaches
GitHub through the mirror, where Linux and Windows binaries are built for the
GitHub release. A `-` in the tag (`v0.2.0-beta.1`) marks a pre-release.

# Why should I use this editor?

If you have to ask, you probably shouldn't.
