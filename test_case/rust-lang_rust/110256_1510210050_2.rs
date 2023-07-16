diff
diff --git a/alacritty/src/renderer/text/atlas.rs b/alacritty/src/renderer/text/atlas.rs
index 9d4cb401..f106c8be 100644
--- a/alacritty/src/renderer/text/atlas.rs
+++ b/alacritty/src/renderer/text/atlas.rs
@@ -157,6 +157,7 @@ impl Atlas {
             // Load data into OpenGL.
             let (format, buffer) = match &glyph.buffer {
                 BitmapBuffer::Rgb(buffer) => {
+                    let buffer: &[u8] = buffer;
                     multicolor = false;
                     // Gles context doesn't allow uploading RGB data into RGBA texture, so need
                     // explicit copy.
@@ -175,6 +176,7 @@ impl Atlas {
                 },
                 BitmapBuffer::Rgba(buffer) => {
                     multicolor = true;
+                    let buffer: &[u8] = buffer;
                     (gl::RGBA, Cow::Borrowed(buffer))
                 },
             };
