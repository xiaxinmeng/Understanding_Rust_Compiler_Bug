diff
> -            1 if ser.content == Content::Message {  }  => Some(ser),
> +            1 if matches!(Content::Message { .. }, ser.content)  => Some(ser),
> 