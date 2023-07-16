
type EngineInit = extern fn (Box<Window>) -> Box<Engine>;
type EngineUpdateAndRender = extern fn (&mut Engine);

fn main() {
    // Open the game as a dynamic library;
    let lib = DynamicLibrary::open(Some(Path::new("gunship-24517baeade73325.dll"))).unwrap();

    let engine_init = unsafe {
        mem::transmute::<*mut EngineInit, EngineInit>(lib.symbol("engine_init").unwrap())
    };

    let engine_update_and_render = unsafe {
        mem::transmute::<*mut EngineUpdateAndRender, EngineUpdateAndRender>(lib.symbol("engine_update_and_render").unwrap())
    };

    let mut window = Window::new("Game Window");
    let mut engine = engine_init(window);
    engine_update_and_render(&mut engine);
}
