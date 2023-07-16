
    let result = result::chain(result): |config| {
        let output_format = getopts::opt_maybe_str(
            match, opt_output_format());
        option::map_default(output_format, result::ok(config)): |output_format| {
            result::chain(parse_output_format(output_format)): |output_format| {
                result::ok({
                    output_format: output_format
                    with config
                })
            }
        }
    };

vs.

    let result = result::chain(result) {|config|
        let output_format = getopts::opt_maybe_str(
            match, opt_output_format());
        option::map_default(output_format, result::ok(config)) {|output_format|
            result::chain(parse_output_format(output_format)) {|output_format|
                result::ok({
                    output_format: output_format
                    with config
                })
            }
        }
    };
