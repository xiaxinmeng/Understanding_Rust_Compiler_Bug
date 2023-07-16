rust
<form>
{%- for setting in settings -%}
  <input type="checkbox"
    {%- if setting == true -%}
      checked
    {- endif -%}
  >
{%- endfor -%}
</form>
