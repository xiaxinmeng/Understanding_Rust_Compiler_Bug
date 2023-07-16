go
// use type assertion to check for nil data pointer of a particular type
p, ok := s.(*S)
if ok {
	isNil = p == nil
}

// use reflection to check for nil data pointer of any type
isNil = reflect.ValueOf(s).IsNil()
