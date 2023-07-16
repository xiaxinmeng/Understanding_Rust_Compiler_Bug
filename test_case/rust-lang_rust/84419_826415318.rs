
error: unable to parse template:

"{% for setting in settings %}\n{{% match setting %}\n{% when Setting::Toggle with {js_data_name, description, default_value} %}\n  <div class=\"setting-line\">\n       <label class=\"toggle\">\n         <input type=\"checkbox\" id=\"{{ js_data_name }} \" >\n         <span class=\"slider\"></span>\n       </label>\n       <div>{{ description }}</div>\n   </div>\n{% else %}\n{% endmatch %}\n{% endfor %}\n</ul>"
