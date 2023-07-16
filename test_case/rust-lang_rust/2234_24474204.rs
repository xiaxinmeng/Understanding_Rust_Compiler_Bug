
block_comment : "/*" block_comment_body * '*' + '/' ;
block_comment_body : non_star * | '*' + non_slash_or_star ;
