
## ibus-dl

This crate provides an unsafe interface to the ibus library installed to the system where it is executed. The ibus shared object is located and loaded at runtime, there shouldn't be any need for any complie time action.

Note that the interface that this crate exposes is limited, so feel free to make a PR if it's missing a function you need.

## Development

The code in this crate is mostly hand written, based on the output from bindgen executed as follows.

```c
// wrapper.h
#include <ibus.h>
```

```
bindgen wrapper.h -o bindings.rs --opaque-type _IBus[A-Z].* --allowlist-function ibus_.* -- `pkg-config --cflags ibus-1.0`
```
