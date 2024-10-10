 this is the "Leptos Axum Starter Template" with a minimal setup to repreoduce the issue.

 `make run` will call the right cargo leptos watch command and serve on 3000

 # the issue

 with the signals wired as they are, whenever the selection of the first select element changes, it would appear that the whole contents of the suspense are redrawn, as:
 - the page flickers
 - value of both selects go back to their defaults
 - "re-evaluating the contents of suspense" is logged to console
 - the following error is logged on console:

 ```
[Signal::update] At Location { file: "src/app.rs", line: 114, col: 31 }, you’re trying to update a Signal<alloc::string::String> (defined at src/app.rs:105:30) that has already been disposed of. This is probably a logic error in a component that creates and disposes of scopes. If it does not cause any issues, it is safe to ignore this warning, which occurs only in debug mode.
 ```

