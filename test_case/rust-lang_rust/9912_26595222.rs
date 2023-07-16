
struct Element: Node { ... }
struct HTMLElement: Element { ... }
struct HTMLIFrameElement: HTMLElement { ... }

trait IElement: Element {
    fn setAttribute(&mut self);
}
