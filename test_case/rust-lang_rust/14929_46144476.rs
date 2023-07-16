 html
<span id='rust-example-raw-8' class='rusttest'>fn main() {
    #![allow(unused_attribute)]
    // Crate ID
    #![crate_id = &quot;projx#2.5&quot;]

    // Additional metadata attributes
    #![desc = &quot;Project X&quot;]
    #![license = &quot;BSD&quot;]
    #![comment = &quot;This is a comment on Project X.&quot;]

    // Specify the output type
    #![crate_type = &quot;lib&quot;]

    // Turn on a warning
    #![warn(non_camel_case_types)]
}</span><pre id='rust-example-rendered-8' class='rust '>
<span class='comment'>// Crate ID
</span><span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>crate_id</span> <span class='op'>=</span> <span class='string'>&quot;projx#2.5&quot;</span>]</span><span class='comment'>

// Additional metadata attributes
</span><span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>desc</span> <span class='op'>=</span> <span class='string'>&quot;Project X&quot;</span>]</span>
<span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>license</span> <span class='op'>=</span> <span class='string'>&quot;BSD&quot;</span>]</span>
<span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>comment</span> <span class='op'>=</span> <span class='string'>&quot;This is a comment on Project X.&quot;</span>]</span><span class='comment'>

// Specify the output type
</span><span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>crate_type</span> <span class='op'>=</span> <span class='string'>&quot;lib&quot;</span>]</span><span class='comment'>

// Turn on a warning
</span><span class='attribute'>\#<span class='op'>!</span>[<span class='ident'>warn</span>(<span class='ident'>non_camel_case_types</span>)]</span>
</pre>
