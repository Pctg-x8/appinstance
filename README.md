## AppInstance

Instantiate once in application anytime, will be destroyed once at application exit

```rust
pub struct Application { main_window: Window }

impl Application
{
	// Instantiated an Application once
	AppInstance!(pub static instance: Application = Application::new());
	fn new() -> Self { ... }
}
// Application::main_window::drop is going to be called at exit
```
