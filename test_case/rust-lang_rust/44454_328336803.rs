Rust
'λ₀...'λₙ lifetimes
τ₀...τₙ types
Trait object-safe
~~WF(for<...> Trait<'λ₀...'λₙ, τ₀...τₙ>) - this part is new~~
WF(for<...> Trait<'λ₀...'λₙ, τ₀...τₙ>: Trait<'λ₀...'λₙ, τ₀...τₙ>) - this part is new
----------------
for<...> Trait<'λ₀...'λₙ, τ₀...τₙ>: for<...> Trait<'λ₀...'λₙ, τ₀...τₙ>
