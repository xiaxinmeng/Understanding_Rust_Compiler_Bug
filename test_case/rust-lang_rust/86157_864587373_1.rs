
<a href='{{page.root_path | safe}}{{krate_with_trailing_slash | safe}}index.html'>
    <div class='logo-container'>
    <img src='
        {%- if layout.logo -%}
        {{layout.logo}}
        {%- else -%}
        {{static_root_path | safe}}rust-logo{{page.resource_suffix}}.png
        {%- endif -%}
        ' alt='logo'>
    </div>
</a>
