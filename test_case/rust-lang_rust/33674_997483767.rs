swift
import Foundation
import Swift

func my_subroutine() {
    print("Hello world" + "\n")
}

do {
    var x: Int32 = 4
    var p = unsafeBitCast(Int(0), to: UnsafeMutablePointer<Int32>.self)
    my_subroutine()
    p.pointee = 3
    print("\(x)" + ", " + "\(p.pointee)" + "\n")
}
