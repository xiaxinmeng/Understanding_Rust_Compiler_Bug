
macro_rules! eval (
    ( $e:tt ) => (ee! ( $e , (id_k) ))
)

macro_rules! ee (
    ( [$(#$x:ident)+] $([$y:ident,$v:tt])*, $k:tt ) =>
        (lookup!($(#$x)+, $([$y,$v])*, $k));

    ( (λ $x:ident. $e:tt) $([$y:ident,$v:tt])*, $k:tt ) =>
        (apply_k! ($k, (closure $x . $e : $([$y,$v])*)));

    ( ($rator:tt $rand:tt) $([$y:ident,$v:tt])*, $k:tt ) =>
        (ee! ( $rator $([$y,$v])*,
              (rator_k $rand $([$y,$v])*, $k)));
)

macro_rules! lookup (
    ( #$x:ident, [$y:ident,$v:tt] $([$ys:ident,$vs:tt])*, $k:tt )
    => (apply_k!($k, $v));

    ( #$x:ident$(#$xs:ident)+, [$y:ident,$v:tt]$([$ys:ident,$vs:tt])+,
     $k:tt )
    => (lookup!($(#$xs)+, $([$ys,($vs)])+, $k));
)

macro_rules! apply_k (
    ( (id_k), $v:tt ) => ( { log_syntax! ( $v ); () } );
    ( (rator_k $rand:tt $([$y:ident,$vs:tt])*, $k:tt), $v:tt ) => (
        ee! ( $rand $([$y,$vs])*,
              (rand_k $v, $k) )
    );
    ( (rand_k (closure $x:ident . $e:tt : $([$y:ident,$vs:tt])*), $k:tt),
       $v:tt ) => (
        ee! ( $e [$x,$v] $([$y,$vs])*, $k )
    )
)

fn main() {
    trace_macros!(true);

    eval! (
        ((λ x. [#x]) (λ y. [#y]))

        // For a good time, call:
        //((λ x. ([#x] [#x])) (λ x. ([#x] [#x])))
    );
}
