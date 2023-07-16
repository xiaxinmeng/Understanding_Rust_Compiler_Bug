rust
use nom::sp;
use nom::types::CompleteStr;
use nom::Convert;
use nom::InputIter;
use nom::{need_more, Compare};
use nom::{Err, IResult, Needed};
pub fn float(i: CompleteStr) -> IResult<CompleteStr, f64> {
    regex::Regex::new("");
    Ok((i, 0.0))
}
pub struct TemperatureAt {
    pub start_h: f64,
    pub start_t: f64,
}
pub fn temperature_at(i: CompleteStr) -> IResult<CompleteStr, TemperatureAt> {
    match {
        let mut sep_res = sp(i);
        match sep_res {
            Ok((i1, _)) => {
                let res: IResult<CompleteStr, TemperatureAt> = match {
                    let sep_res = sp(i1);
                    match sep_res {
                        Ok((i2, _)) => {
                            i2.compare("");
                            let res2: IResult<_, CompleteStr> = need_more(i2, Needed::Size(0));
                            res2
                        }
                        Err(e07) => Err(Err::convert(e07)),
                    }
                } {
                    Err(e06) => Err(e06),
                    Ok(_) => match match sep_res {
                        Ok(_) => {
                            let res2: IResult<_, char> = match i1.iter_elements().next() {
                                Some('A') => {
                                    let e05 = nom::ErrorKind::Char;
                                    Err(Err::Error(nom::Context::Code(i1, e05)))
                                }
                                _ => need_more(i1, Needed::Size(1)),
                            };
                            res2
                        }
                        Err(e08) => Err(e08),
                    } {
                        Err(e09) => Err(e09),
                        _ => match {
                            sep_res = sp(i);
                            match sep_res {
                                Err(e10) => Err(e10),
                                _ => {
                                    let res = float(i1);
                                    res
                                }
                            }
                        } {
                            Err(e11) => Err(e11),
                            Ok((i7, o)) => match {
                                sep_res = sp(i7);
                                match sep_res {
                                    Ok((i5, _)) => {
                                        let res =
                                            need_more::<_, CompleteStr, u32>(i5, Needed::Size(1));
                                        res
                                    }
                                    Err(e12) => Err(Err::convert(e12)),
                                }
                            } {
                                Err(e01) => Err(e01),
                                _ => match {
                                    sep_res = sp(i7);
                                    sep_res
                                } {
                                    Err(e02) => Err(e02),
                                    _ => Ok((
                                        i,
                                        TemperatureAt {
                                            start_h: 0.0,
                                            start_t: 0.0,
                                        },
                                    )),
                                },
                            },
                        },
                    },
                };
                drop(&res);
                res
            }
            Err(e03) => Err(e03),
        }
    } {
        Err(e04) => Err(e04),
        Ok((i6, o01)) => loop {},
    }
}
