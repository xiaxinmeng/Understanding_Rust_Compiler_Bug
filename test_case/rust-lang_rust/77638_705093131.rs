diff
    pub fn checked_add<Dur: Duration>(self, duration: Dur) -> Option<Self>
    where
        Dur: FixedPoint,
-        Clock::T: TryFrom<Dur::T>
+        Clock::T: TryFrom<Dur::T> + ops::Div<Output=Clock::T>
