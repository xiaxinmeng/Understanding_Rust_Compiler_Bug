
Local(Trait<T2...Tn> for T1) =
    Trait is local || (
       exists i. Local(Ti) &&
       forall i. Ti is not a type parameter
    )
Local(Type<T1...Tn>) =
    Type is local || (
       exists i. Local(Ti) &&
       forall i. Ti is not a type parameter
    )
