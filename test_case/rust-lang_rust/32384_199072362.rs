
<eddyb> WindowsBunny: so you're telling me if I don't have debuginfo for an instruction
<eddyb> it can't be resolved back to a function?
<WindowsBunny> yep
<WindowsBunny> eddyb: The names of functions internal to an object file aren't stored anywhere
<WindowsBunny> there is nothing for the linker to dig up to shove into the .pdb
