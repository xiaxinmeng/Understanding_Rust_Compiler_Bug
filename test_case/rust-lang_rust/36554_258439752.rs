
self.sess.span_err(var_origin.span, "cannot infer value for type parameter `{}`", var_origin.name)
        .emit()
