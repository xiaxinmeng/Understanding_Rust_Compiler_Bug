
Maybe<wr::WrSpatialId> DisplayListBuilder::PushStackingContext(
    const wr::StackingContextParams& aParams, const wr::LayoutRect& aBounds,
    const wr::RasterSpace& aRasterSpace) {
  MOZ_ASSERT(mClipChainLeaf.isNothing(),
             "Non-empty leaf from clip chain given, but not used with SC!");

  wr::LayoutTransform matrix;
  const gfx::Matrix4x4* transform = aParams.mTransformPtr;
  if (transform) {
    matrix = ToLayoutTransform(*transform);
  }
  const wr::LayoutTransform* maybeTransform = transform ? &matrix : nullptr;
  WRDL_LOG("PushStackingContext b=%s t=%s\n", mWrState,
           ToString(aBounds).c_str(),
           transform ? ToString(*transform).c_str() : "none");
  printf("XXX before %ld %ld\n", aParams.mFilters.Length(), aParams.mFilterDatas.Length());
  wr_dp_push_stacking_context(
      aBounds,
      123456789,
      aRasterSpace,
      &aParams);
  return Nothing();
}
