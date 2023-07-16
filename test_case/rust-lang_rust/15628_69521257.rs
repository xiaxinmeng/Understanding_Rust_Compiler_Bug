 rust
    fn word_at(&self, idx: usize) -> &str {
        let s = &self[idx..];
        let mut take_prev = true;
        let mut take_cur = true;
        let mut state = WState::Start;
        let mut prev_idx = 0;
        let mut idx = 0;

        for (curr, c) in s.char_indices() {
            prev_idx = idx;
            idx = curr;

            let cat = word::category(c);
            if cat == WCat::Extend || cat == WCat::Format {
                if state == WState::Start {
                    break;
                } else {
                    continue;
                }
            }

            state = match state {
                WState::Start if c == '\r' => {
                    if idx + 1 != s.len() && s.char_at(idx + 1) == '\n' {
                        idx += 1;
                    }
                    break;
                },
                WState::Start => match cat {
                    WCat::ALetter           => WState::ALetter,
                    WCat::HebrewLetter      => WState::HLetter,
                    WCat::Numeric           => WState::Numeric,
                    WCat::Katakana          => WState::Katakana,
                    WCat::ExtendNumLet      => WState::ExtendNumLet,
                    WCat::RegionalIndicator => WState::RegionalIndicator,
                    _ => break,
                },
                WState::ALetter => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidLetter | WCat::MidNumLet |
                        WCat::SingleQuote => WState::ALetterExt,
                    WCat::Numeric => WState::Numeric,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::ALetterExt => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    _ => {
                        take_cur = false;
                        take_prev = false;
                        break;
                    }
                },
                WState::HLetter => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidLetter | WCat::MidNumLet => WState::HLetterExt(0),
                    WCat::SingleQuote => WState::HLetterExt(1),
                    WCat::DoubleQuote => WState::HLetterExt(2),
                    WCat::Numeric => WState::Numeric,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::HLetterExt(n) => match cat {
                    WCat::ALetter if n < 2 => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    _ => {
                        take_cur = false;
                        take_prev = n == 1;
                        break;
                    }
                },
                WState::Numeric => match cat {
                    WCat::Numeric => WState::Numeric,
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidNum | WCat::MidNumLet |
                        WCat::SingleQuote => WState::NumericExt,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::NumericExt => match cat {
                    WCat::Numeric => WState::Numeric,
                    _ => {
                        take_cur = false;
                        take_prev = false;
                        break;
                    }
                },
                WState::Katakana => match cat {
                    WCat::Katakana => WState::Katakana,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::ExtendNumLet => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::Numeric => WState::Numeric,
                    WCat::Katakana => WState::Katakana,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::RegionalIndicator => match cat {
                    WCat::RegionalIndicator => WState::RegionalIndicator,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::SingleQuote => unreachable!(),
            }
        }

        if take_cur {
            idx += s.char_at(idx).len_utf8();
        } else if !take_prev {
            idx = prev_idx;
        }

        &s[..idx]
    }

    fn word_at_reverse(&self, idx: usize) -> &str {
        let s = &self[..idx];

        let mut take_prev = true;
        let mut take_cur = true;
        let mut state = WState::Start;
        let mut prev_prev_idx = s.len();
        let mut prev_idx = s.len();
        let mut idx = s.len();

        for (curr, c) in s.char_indices().rev() {
            let cat = word::category(c);
            if cat == WCat::Extend || cat == WCat::Format {
                continue;
            }

            prev_prev_idx = prev_idx;
            prev_idx = idx;
            idx = curr;

            state = match state {
                WState::Start if c == '\n' => {
                    if idx > 0 && s.char_at_reverse(idx) == '\r' {
                        idx -= 1;
                    }
                    break;
                },
                WState::Start => match cat {
                    WCat::ALetter           => WState::ALetter,
                    WCat::HebrewLetter      => WState::HLetter,
                    WCat::SingleQuote       => WState::SingleQuote,
                    WCat::Numeric           => WState::Numeric,
                    WCat::Katakana          => WState::Katakana,
                    WCat::ExtendNumLet      => WState::ExtendNumLet,
                    WCat::RegionalIndicator => WState::RegionalIndicator,
                    _ => break,
                },
                WState::ALetter => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidLetter | WCat::MidNumLet |
                        WCat::SingleQuote => WState::ALetterExt,
                    WCat::Numeric => WState::Numeric,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::ALetterExt => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    _ => {
                        take_cur = false;
                        take_prev = false;
                        break;
                    }
                },
                WState::HLetter => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidLetter | WCat::MidNumLet => WState::HLetterExt(0),
                    WCat::DoubleQuote => WState::HLetterExt(2),
                    WCat::Numeric => WState::Numeric,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::HLetterExt(n) => match cat {
                    WCat::ALetter if n < 2 => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    _ => {
                        take_cur = false;
                        take_prev = false;
                        break;
                    }
                },
                WState::SingleQuote => match cat {
                    WCat::HebrewLetter => WState::HLetter,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::Numeric => match cat {
                    WCat::Numeric => WState::Numeric,
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::MidNum | WCat::MidNumLet |
                        WCat::SingleQuote => WState::NumericExt,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::NumericExt => match cat {
                    WCat::Numeric => WState::Numeric,
                    _ => {
                        take_cur = false;
                        take_prev = false;
                        break;
                    }
                },
                WState::Katakana => match cat {
                    WCat::Katakana => WState::Katakana,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::ExtendNumLet => match cat {
                    WCat::ALetter => WState::ALetter,
                    WCat::HebrewLetter => WState::HLetter,
                    WCat::Numeric => WState::Numeric,
                    WCat::Katakana => WState::Katakana,
                    WCat::ExtendNumLet => WState::ExtendNumLet,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
                WState::RegionalIndicator => match cat {
                    WCat::RegionalIndicator => WState::RegionalIndicator,
                    _ => {
                        take_cur = false;
                        break;
                    }
                },
            }
        }

        if idx == s.len() {
            idx = 0;
        } else if !take_prev {
            idx = prev_prev_idx;
        } else if !take_cur {
            idx = prev_idx;
        }

        &s[idx..]
    }
