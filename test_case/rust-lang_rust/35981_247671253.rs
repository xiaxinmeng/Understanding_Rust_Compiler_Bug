 go
func f5(s []int) {
    for i := range s {
        _ = s[i]
        _ = s[i:len(s)]
        _ = s[:i+1]
    }
}
