rust
let stylesheet = ...;
let node = rsx! {
  <view style={stylesheet.get(".root")}>
      Hello world!
  </view>
};
