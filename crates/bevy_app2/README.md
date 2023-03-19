# bevy_app2

This was an idea I had where rather than making plugins and apps a special case, they're just a special kind of system.
I've attempted to address some of the issues I've had with the current app system at once. This is a proof of concept
and not meant to be merged. Next steps, if this is something we would want to pursue, would be to draft an RFC with more
detailed implementation and migration considerations.

## TLDR

- `App` is a `Runtime` and a `Runner`
- `Runtime` is a `World` and a `ScheduleLabel`. It is a `Component`; this enables worlds.

## Why?

Using systems to manage plugin setup unlocks a lot of flexibility. It means:

- plugins can be added and removed in any order
- plugins can be added and removed in parallel
- plugins can be used in a `World` without an `App` (multiple/nested worlds; `World` can be a `Component`)
- plugins can (in theory) be added and removed at runtime

