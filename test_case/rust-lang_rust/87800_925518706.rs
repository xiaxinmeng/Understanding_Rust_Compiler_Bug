rust
pub fn map_interpolation(
    interp: (&str, Span),
    sampling: Option<(crate::Sampling, Span)>,
) -> Result<crate::Interpolation, Error<'_>> {
    // Option::unzip is not stable.
    let (sampling, sampling_span) = match sampling {
        Some((sampling, span)) => (Some(sampling), Some(span)),
        None => (None, None),
    };
    match interp.0 {
        "linear" => Ok(crate::Interpolation::Linear(
            sampling.unwrap_or(crate::Sampling::Center),
        )),
        "flat" => {
            if let Some(span) = sampling_span {
                return Err(Error::SamplingNotPermitted {
                    interpolation: interp.1,
                    sampling: span,
                });
            }
            Ok(crate::Interpolation::Flat)
        }
        "perspective" => Ok(crate::Interpolation::Perspective(
            sampling.unwrap_or(crate::Sampling::Center),
        )),
        _ => Err(Error::UnknownInterpolation(interp.1)),
    }
}
