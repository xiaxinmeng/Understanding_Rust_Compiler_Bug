rust
work $ cd servo/
components/layout_thread/lib.rs
1632:        self.generation.set(self.generation.get() + 1);

components/script/timers.rs
250:        self.suspension_offset.set(self.suspension_offset.get() + additional_offset);

components/script/dom/webglshader.rs
190:        self.attached_counter.set(self.attached_counter.get() + 1);
195:        self.attached_counter.set(self.attached_counter.get() - 1);

components/script/dom/window.rs
1580:        self.pending_reflow_count.set(self.pending_reflow_count.get() + 1);

components/script/dom/node.rs
257:        self.children_count.set(self.children_count.get() + 1);
297:        self.children_count.set(self.children_count.get() - 1);

components/script/dom/htmllinkelement.rs
295:        self.request_generation_id.set(self.request_generation_id.get().increment());
319:        self.pending_loads.set(self.pending_loads.get() + 1)
328:        self.pending_loads.set(self.pending_loads.get() - 1);

components/script/dom/htmlmediaelement.rs
680:        self.generation_id.set(self.generation_id.get() + 1);

components/script/dom/htmlstyleelement.rs
202:        self.pending_loads.set(self.pending_loads.get() + 1)
211:        self.pending_loads.set(self.pending_loads.get() - 1);

components/script/dom/htmlimageelement.rs
603:        self.generation.set(self.generation.get() + 1);

components/script/dom/document.rs
551:        self.dom_count.set(self.dom_count.get() + 1);
556:        self.dom_count.set(self.dom_count.get() - 1);
1384:        count_cell.set(count_cell.get() + 1);
1390:        count_cell.set(count_cell.get() - 1);
1507:                self.spurious_animation_frames.set(self.spurious_animation_frames.get() + 1)

components/gfx/tests/font_context.rs
87:        self.find_font_count.set(self.find_font_count.get() + 1);
