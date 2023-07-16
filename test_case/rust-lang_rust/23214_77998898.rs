 rust
    #[test]
    fn udp_shutdown_smoke() {
        each_ip(&mut |addr, _| {
            let sock = t!(UdpSocket::bind(&addr));

            let _t = thread::scoped(|| {
                let mut sock = &sock;
                assert!(sock.recv_from(&mut []).is_err());
            });

            t!(sock.shutdown(Shutdown::Read));
        })
    }
