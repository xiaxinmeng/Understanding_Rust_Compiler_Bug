swift
@_cdecl("platform_panic")
func platform_panic() {
    DispatchQueue.global(qos: .userInteractive)
        .sync {
            fatalError()
        }
}
