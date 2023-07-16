rust
impl TryConvert<TFlt> for TNum {
    fn try_convert(self) -> Result<TFlt, ConvertionError> {
        let ret = self as TFlt;
        if self == (ret as TNum) {
            Ok(ret)
        } else {
            Err(ConvertionError::out_of_precision::<TNum, TFlt>(self, ret))
        }
    }
}
