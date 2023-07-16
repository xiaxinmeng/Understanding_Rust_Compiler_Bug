
@@ -3670,17 +3683,23 @@ MacroExpander::transcribe_rule (
            case ContextType::ITEM: {
              auto item = parser.parse_item (true);
              if (item != nullptr && !parser.has_errors ())
-               {
-                 rust_debug ("HELLO WORLD: [%s]", item->as_string ().c_str ());
-                 nodes.push_back (std::move (item));
-               }
+               nodes.push_back (std::move (item));
            }
            break;
 
            case ContextType::BLOCK: {
-             auto expr = parser.parse_expr ();
-             if (expr != nullptr && !parser.has_errors ())
-               nodes.push_back (std::move (expr));
+             if (semicolon)
+               {
+                 auto stmt = parser.parse_stmt ();
+                 if (stmt != nullptr && !parser.has_errors ())
+                   nodes.push_back (std::move (stmt));
+               }
+             else
+               {
+                 auto expr = parser.parse_expr ();
+                 if (expr != nullptr && !parser.has_errors ())
+                   nodes.push_back (std::move (expr));
+               }
            }
            break;
          }

