# Rust-one
Is rust language really safe?

# Definition of Safe
"Safety" is the state of being "safe" (from french - sauf), the condition of being protected against damages or undesired results.

# Why safety in Software matters?
- The Ariane 5 Disaster -> https://www.bugsnag.com/blog/bug-day-ariane-5-disaster | https://www.youtube.com/watch?v=5tJPXYA0Nec
- Race conditions in Therac-25 (radiotherapy) -> https://www.bugsnag.com/blog/bug-day-race-condition-therac-25 | https://www.youtube.com/watch?v=Ap0orGCiou8
- Mars Climate Orbiter -> https://www.bugsnag.com/blog/bug-day-mars-climate-orbiter | https://www.youtube.com/watch?v=lcYkOh4nweE

# Rust Memory Safety Model
- No null pointer dereferences
- No dangling pointers
- No iterator invalidation
- No use after free
- No double frees
- No out of bound access
- No buffer overflows
- No data races