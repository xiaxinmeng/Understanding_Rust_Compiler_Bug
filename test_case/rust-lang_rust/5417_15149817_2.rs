
trait Enum<I: BaseIter<Self>> : Eq + Ord + TotalOrd
                              + Enumerable<I>
                              + FromInt + ToInt
                              + FromStr + ToStr { }
