# await-issue

Tried to SSR-render results of a server fn in an <Await> which in turn contained a `move` lambda for a reactive signal, and got stuck.

Specifically I couldn't make the (outer) closure a Fn rather than a FnOnce.

what I get:
```
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
   --> src/app.rs:87:32
    |
78  | /      view! {
79  | |          <Await
80  | |              future=|| load_fields()
81  | |              let:initial_data
...   |
87  | |                      prop:value=move || {
    | |                      ----       -^^^^^^
    | |                      |          |
    | | _____________________|__________this closure implements `FnOnce`, not `Fn`
    | ||                     |
    | ||                     required by a bound introduced by this call
88  | ||                         let existing_data = initial_data
    | ||                                             ------------ closure is `FnOnce` because it moves the variable `*initial_data` out of its environment
89  | ||                             .unwrap()
90  | ||                             .map_or("".to_string(), |s| s.name);
...   ||
96  | ||                             )
97  | ||                     }
    | ||_____________________- this tail expression is of type `{closure@app.rs:87:32}`
...   |
105 | |          </Await>
106 | |      }
    | |______- the requirement to implement `Fn` derives from here
```