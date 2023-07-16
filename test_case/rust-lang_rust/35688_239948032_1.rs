
{
   "message" : "expected token: `,`",
   "level" : "error",
   "rendered" : null,
   "children" : [],
   "code" : null,
   "spans" : [
      {
         "column_end" : 59,
         "is_primary" : true,
         "column_start" : 28,
         "expansion" : {
            "span" : {
               "line_end" : 13,
               "byte_start" : 498,
               "label" : null,
               "file_name" : "src/test/ui/codemap_tests/bad-format-args.rs",
               "text" : [
                  {
                     "highlight_start" : 5,
                     "highlight_end" : 19,
                     "text" : "    format!(\"\" 1);"
                  }
               ],
               "expansion" : null,
               "column_start" : 5,
               "suggested_replacement" : null,
               "byte_end" : 512,
               "line_start" : 13,
               "column_end" : 19,
               "is_primary" : false
            },
            "def_site_span" : {
               "file_name" : "<std macros>",
               "text" : [
                  {
                     "highlight_start" : 1,
                     "highlight_end" : 28,
                     "text" : "( $ ( $ arg : tt ) * ) => ("
                  },
                  {
                     "highlight_start" : 1,
                     "highlight_end" : 63,
                     "text" : "$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )"
                  }
               ],
               "label" : null,
               "byte_start" : 2660,
               "line_end" : 2,
               "is_primary" : false,
               "column_end" : 63,
               "line_start" : 1,
               "byte_end" : 2750,
               "expansion" : null,
               "column_start" : 1,
               "suggested_replacement" : null
            },
            "macro_decl_name" : "format!"
         },
         "suggested_replacement" : null,
         "byte_end" : 2746,
         "line_start" : 2,
         "text" : [
            {
               "text" : "$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )",
               "highlight_start" : 28,
               "highlight_end" : 59
            }
         ],
         "file_name" : "<std macros>",
         "line_end" : 2,
         "byte_start" : 2715,
         "label" : null
      }
   ]
}
