
<njn> Ms2ger: that's definitely an improvement
<njn> Ms2ger: I suggest changing the method name to something other than `method`, because that becomes unclear in the representation section
<njn> Ms2ger: also, is the representation section important to be able to use trait objects? If not, a "you don't really need to know these details" warning would be helpful
<huon> njn: it is helpful to understand certain errors (particularly those around object safety)
<huon> but that's all, AFAIK
