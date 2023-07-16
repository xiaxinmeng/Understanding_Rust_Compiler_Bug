
313 |           vs_imgs: &'a SmallVec<[&ImageResource; ::MAX_SHADERSTAGE_IMAGES]>,
    |                                  -------------- these two types are declared with different lifetimes...
314 |           fs_imgs: &'a SmallVec<[&ImageResource; ::MAX_SHADERSTAGE_IMAGES]>,
    |                                  --------------
...
320 |               let imgs = if stage_index == (ShaderStage::Vertex as usize) {
    |  ________________________^
321 | |                 vs_imgs
322 | |             } else {
323 | |                 fs_imgs
324 | |             };
    | |_____________^ ...but data from `vs_imgs` flows into `fs_imgs` here

