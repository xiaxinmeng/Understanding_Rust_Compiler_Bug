
  171|      1|    let short_used_not_covered_closure_line_break_no_block_embedded_branch =
  172|      1|        | _unused_arg: u8 |
  173|       |            println!(
  174|       |                "not called: {}",
  175|       |                if is_true { "check" } else { "me" }
  176|       |            )
  177|       |    ;
  178|       |
  179|      1|    let short_used_not_covered_closure_line_break_block_embedded_branch =
  180|      1|        | _unused_arg: u8 |
  181|      0|        {
  182|      0|            println!(
  183|      0|                "not called: {}",
  184|      0|                if is_true { "check" } else { "me" }
  185|       |            )
  186|      0|        }
  187|       |    ;
