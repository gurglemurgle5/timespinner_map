# Timespinner Map
Currently only a map viewer for Timespinner. Not planned to make into anything like a level editor at the current
moment, maybe if it's something people really want.

## Setting up
1. Get Rust: https://www.rust-lang.org/tools/install
2. Find a program that converts XNB files to PNG files. Shouldn't matter which one. If you know of a good, easy to use
  one that works on Linux, please let me know and I can add it here. If you're good with a CLI, I can recommend
  https://github.com/LeonBlade/xnbcli.
3. Dump all of the XNB files in Timespinner's `Content` into a `Content` directory in this project.
4. Run with `cargo run --release -- TIMESPINNER_PATH` where `TIMESPINNER_PATH` is the directory where you have
  Timespinner installed.

You can pan around with the mouse, either by clicking and dragging, middle clicking and dragging, or scrolling. No other
controls are implimented yet.

## Notes
The camera starts at (0, 0). All rooms are placed down-right of that. The present is the closest to the origin, with the
past being below the present and ??? being being below the past.

## TODO
- Backgrounds / related
- Enemies / Items / Others
- Render out rooms to images / external image editing program
- Maybe some additional debugging info?
- Anything else labeled with a TODO in the code
- Might be nice to figure out text rendering as that would be really useful
- Some tiles / other things are conditional, impliment that
- Less panics, more clean error handling
- Maybe directly read XNB files instead of requiring them to be dumped
- Zooming in/out
- General code cleanup

## Not Planned
- Automatic allignment of rooms (unfortunately the game does not make physical sense)
