diff
16	LL | fn test<T: Sized>(_: T) {}
17	   |            ----- predicate
18	
-	error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+	error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
20	  --> $DIR/issue-85360-eval-obligation-ice.rs:13:5
21	   |
22	LL |     test::<MaskedStorage<GenericComp2<Pos>>>(make());

25	LL | fn test<T: Sized>(_: T) {}
26	   |         - predicate
27	
-	error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOkModuloRegions)
+	error: evaluate(Binder(TraitPredicate(<MaskedStorage<GenericComp2<Pos>> as std::marker::Sized>, polarity:Positive), [])) = Ok(EvaluatedToOk)
29	  --> $DIR/issue-85360-eval-obligation-ice.rs:13:5
