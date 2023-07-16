rust
    macro_rules! assert_option_parses {
        ($opt:expr, $data:expr) => ({
            assert_eq!(TcpOption::parse($data), Ok((&[][..], $opt)));
            let buffer = &mut [0; 40][..$opt.buffer_len()];
            assert_eq!($opt.emit(buffer), &mut []);
            assert_eq!(&*buffer, $data);
        })
    }

    #[test]
    fn test_tcp_options() {
        assert_option_parses!(TcpOption::EndOfList,
                              &[0x00]);
        assert_option_parses!(TcpOption::NoOperation,
                              &[0x01]);
        assert_option_parses!(TcpOption::MaxSegmentSize(1500),
                              &[0x02, 0x04, 0x05, 0xdc]);
        assert_option_parses!(TcpOption::WindowScale(12),
                              &[0x03, 0x03, 0x0c]);
        assert_option_parses!(TcpOption::SackPermitted,
                              &[0x4, 0x02]);
        assert_option_parses!(TcpOption::SackRange([Some((500, 1500)), None, None]),
                              &[0x05, 0x0a,
                                0x00, 0x00, 0x01, 0xf4, 0x00, 0x00, 0x05, 0xdc]);
        assert_option_parses!(TcpOption::SackRange([Some((875, 1225)), Some((1500, 2500)), None]),
                              &[0x05, 0x12,
                                0x00, 0x00, 0x03, 0x6b, 0x00, 0x00, 0x04, 0xc9,
                                0x00, 0x00, 0x05, 0xdc, 0x00, 0x00, 0x09, 0xc4]);
        assert_option_parses!(TcpOption::SackRange([Some((875000, 1225000)),
                                                    Some((1500000, 2500000)),
                                                    Some((876543210, 876654320))]),
                              &[0x05, 0x1a,
                                0x00, 0x0d, 0x59, 0xf8, 0x00, 0x12, 0xb1, 0x28,
                                0x00, 0x16, 0xe3, 0x60, 0x00, 0x26, 0x25, 0xa0,
                                0x34, 0x3e, 0xfc, 0xea, 0x34, 0x40, 0xae, 0xf0]);
        assert_option_parses!(TcpOption::Unknown { kind: 12, data: &[1, 2, 3][..] },
                              &[0x0c, 0x05, 0x01, 0x02, 0x03])
    }
