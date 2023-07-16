diff --git a/voxygen/src/hud/minimap.rs b/voxygen/src/hud/minimap.rs
index 990c45172..5b4b3f12a 100644
--- a/voxygen/src/hud/minimap.rs
+++ b/voxygen/src/hud/minimap.rs
@@ -79,9 +79,9 @@ impl VoxelMinimap {
                                 .ok()
                                 .and_then(|block| block.get_color())
                                 .map(|rgb| [rgb.r, rgb.g, rgb.b, 192])
-                                .map(|c| Vec4::<u8>::from(c).as_())
+                                .map(|c| Vec4::<u8>::from(c))
                                 .unwrap_or(Vec4::one());
-                            rgba += color.as_() * weights[z as usize];
+                            rgba += color.as_() * weights[dz as usize];
                         }
                         let rgba: Vec4<u8> = (rgba / weights.iter().sum::<u16>()).as_();
                         rgba
@@ -131,14 +131,14 @@ impl VoxelMinimap {
                             let rgba: Vec4<u8> = (rgba / weights.iter().sum::<u16>()).as_();
                             image::Rgba([rgba.x, rgba.y, rgba.z, rgba.w])
                         };*/
-                        let color = self
+                        let color: Vec4<u8> = self
                             .chunk_minimaps
                             .get(&(cpos + coff))
                             .and_then(|(zlo, g)| g.get((pos.z as i32 - zlo) as usize))
                             .and_then(|grid| grid.get(cmod).map(|c| c.as_()))
                             .unwrap_or(Vec4::one());
                         self.composited
-                            .put_pixel(x, VOXEL_MINIMAP_SIDELENGTH - y - 1, color);
+                            .put_pixel(x, VOXEL_MINIMAP_SIDELENGTH - y - 1, image::Rgba([color.x, color.y, color.z, color.w]));
                     }
                 }
