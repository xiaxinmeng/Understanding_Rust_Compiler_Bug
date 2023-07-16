
<ul>
{% for setting in settings %}
{% match setting %}
{% when Setting::Toggle with {js_data_name, description, default_value} %}
  <div class="setting-line">
       <label class="toggle">
         <input type="checkbox" id="{{ js_data_name }} " >
         <span class="slider"></span>
       </label>
       <div>{{ description }}</div>
   </div>
{% else %}
{% endmatch %}
{% endfor %}
</ul>
