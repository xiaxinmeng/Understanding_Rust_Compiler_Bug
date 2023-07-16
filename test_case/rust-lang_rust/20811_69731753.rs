
BitOrExpr = BitXorExpr | BitOrExpr "^" BitXorExpr
RangeExpr = BitOrExpr | ".." BitOrExpr | BitOrExpr ".." | BitOrExpr ".." BitOrExpr
CompExpr = RangeExpr | RangeExpr ("==" | "!=" | "<" | ">" | "<=" | ">=") RangeExpr
AndExpr = CompExpr | AndExpr "&&" CompExpr
