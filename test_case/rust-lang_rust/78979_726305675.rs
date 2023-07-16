
> 	/* Ensure offsets don't wrap. */
> 	if (pos_in + count < pos_in || pos_out + count < pos_out)
> 		return -EOVERFLOW;
> 