rust

pub fn with_xxx_task<C, A, R, HCX>(&self,
                                   key: DepNode,
                                   cx: C,
                                   arg: A,
                                   task: fn(C, A) -> R)
                                   -> (R, DepNodeIndex) 
{
    self.with_task(key, cx, arg, |cx, arg| {
        // Explicitly add the read to the Krate node
        self.read(DepNode::Krate);

        // Execute the task without recording any other reads
        self.with_ignore(|| {
            task(cx, arg)
        })     
    })
}
