
forall<P1...> {
    if (FromEnv(WC_trait)) {
        WellFormed(InputTypes(WC_trait)) &&

            forall<P2...> {
                if (FromEnv(WC_assoc)) {
                    WellFormed(InputTypes(Bounds_assoc)) &&
                        WellFormed(InputTypes(WC_assoc))
                }
            }
    }
}
