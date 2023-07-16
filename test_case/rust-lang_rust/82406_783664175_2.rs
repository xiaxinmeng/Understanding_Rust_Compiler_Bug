rust
mod collection_ops {
    use crate::tuple::{snd, swap};
    use dc2::{key::Key, monoid::Monoid, Collection, Op, Relation};
    use std::ops::Mul;

    pub trait SemiJoinOn<'a, D1: Key, D2, R: Monoid> {
        fn semijoin_on<F: Fn(&D1) -> D2 + 'static, C2: Op<D = D2, R = R>>(
            self,
            other: Relation<'a, C2>,
            f: F,
        ) -> Collection<'a, D1, R>;
    }

    impl<'a, D1: Key, D2: Key, R: Monoid + Mul<R, Output = R>, C: Op<D = D1, R = R>>
        SemiJoinOn<'a, D1, D2, R> for Relation<'a, C>
    {
        fn semijoin_on<F: Fn(&D1) -> D2 + 'static, C2: Op<D = D2, R = R>>(
            self,
            other: Relation<'a, C2>,
            f: F,
        ) -> Collection<'a, D1, R> {
            self.map(move |val| (f(&val), val))
                .semijoin(other)
                .map(snd)
                .collect()
        }
    }

    pub trait SemiJoinOnSnd<'a, D1: Key, D2: Key, R: Monoid> {
        fn semijoin_on_snd<C2: Op<D = D2, R = R>>(
            self,
            other: Relation<'a, C2>,
        ) -> Collection<'a, (D1, D2), R>;
    }

    impl<'a, D1: Key, D2: Key, R: Monoid + Mul<R, Output = R>, C: Op<D = (D1, D2), R = R>>
        SemiJoinOnSnd<'a, D1, D2, R> for Relation<'a, C>
    {
        fn semijoin_on_snd<C2: Op<D = D2, R = R>>(
            self,
            other: Relation<'a, C2>,
        ) -> Collection<'a, (D1, D2), R> {
            self.map(swap).semijoin(other).map(swap).collect()
        }
    }
}
mod ops {
    use std::hash::{Hash, Hasher};
    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct RcRaw<T>(pub Rc<T>);

    impl<T> Clone for RcRaw<T> {
        fn clone(&self) -> Self {
            RcRaw(Rc::clone(&self.0))
        }
    }

    impl<T> PartialEq for RcRaw<T> {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    impl<T> Eq for RcRaw<T> {}

    impl<T> Hash for RcRaw<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            Rc::as_ptr(&self.0).hash(state);
        }
    }

    impl<T> Deref for RcRaw<T> {
        type Target = T;
        fn deref(&self) -> &T {
            Rc::deref(&self.0)
        }
    }
}
mod primitives {
    use std::cmp::Ordering;
    use std::ops::Not;
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct Assig(isize);
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub struct Var(isize);
    pub type MicroLevel = usize;
    impl Assig {
        pub fn var(self) -> Var {
            Var(self.0.abs())
        }
    }
    impl Not for Assig {
        type Output = Assig;
        fn not(self) -> Self::Output {
            Assig(-self.0)
        }
    }
    impl PartialOrd for Assig {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Assig {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.0.abs().cmp(&other.0.abs()) {
                Ordering::Equal => self.0.cmp(&other.0),
                res => res,
            }
        }
    }
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
    pub struct RuleIndex(usize);
}
mod program {
    use crate::primitives::{Assig, RuleIndex, Var};
    use dc2::Collection;

    #[derive(Clone)]
    pub struct RulesCollections {
        pub rule_index: Collection<'static, RuleIndex>,
        pub rule: Collection<'static, (RuleIndex, Assig)>,
        pub vars: Collection<'static, Var>,
    }
}
mod solver {
    mod binary {
        use super::learnt::RefRule;
        use super::primitives::DecisionLevel;
        use super::SolverCollections;
        use crate::collection_ops::{SemiJoinOn, SemiJoinOnSnd};
        use crate::primitives::{Assig, RuleIndex, Var};
        use crate::tuple::{fst, snd, swap};
        use dc2::key::Key;
        use dc2::map::{AssertOnes, VecMap};
        use dc2::{
            Arrangement, Collection, CreationContext, Input, MapMapArrangement, MappingArrangement,
        };
        use std::collections::hash_map::DefaultHasher;
        use std::collections::{BTreeMap, HashMap};
        use std::hash::{Hash, Hasher};
        pub struct Binary {
            pub binary_input: Input<((Assig, Assig), (RefRule, DecisionLevel))>,
            pub binary_output: MapMapArrangement<Assig, (Assig, Path)>,
            pub binary_by_level: MapMapArrangement<DecisionLevel, ((Assig, Assig), RefRule)>,
            pub closure: MappingArrangement<(Assig, Assig), usize>,
            pub self_implied: Arrangement<
                (usize, (Var, Assig)),
                isize,
                BTreeMap<usize, HashMap<Var, HashMap<Assig, isize>>>,
            >,
            pub other_un_impls: MapMapArrangement<Var, (Assig, RuleIndex)>,
            pub other_bin_impls: MapMapArrangement<(Assig, Assig), RuleIndex>,
        }
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum Path {
            Direct(RuleIndex),
            Indirect(RefRule),
        }
        impl SolverCollections<'_> {
            pub fn make_binary(&self) -> Binary {
                let active = self.active.get();
                let rule_sizes = self.rule_sizes.get().clone();
                let binary_rule_inds = rule_sizes
                    .filter(|&(_key, count)| count == 2)
                    .map(fst)
                    .named("binary_rule_inds");
                let binary_rules = active
                    .rule
                    .clone()
                    .semijoin(binary_rule_inds)
                    .reduce(|_i, xs: &HashMap<Assig, isize>| {
                        let mut iter = xs.iter().assert_ones();
                        let &x = iter.next().expect("No assigs");
                        let &y = iter.next().expect("Only 1 assig");
                        assert!(iter.next().is_none(), "More than 2 assigs");
                        VecMap::new(vec![((!x, y), 1), ((!y, x), 1)])
                    })
                    .map(|(i, imp)| (imp, Path::Direct(i)))
                    .named("binary_rules");
                let (binary_input, binary_manual) = self.context.borrow().create_input();
                let binary_manual = binary_manual.split().named("binary_manual");
                let binary_by_level: MapMapArrangement<DecisionLevel, ((Assig, Assig), RefRule)> =
                    binary_manual
                        .clone()
                        .map(|(xy, (i, dl))| (dl, (xy, i)))
                        .dynamic()
                        .named("binary_by_level")
                        .get_arrangement(&self.context.borrow());
                let verts = self.rem_lits.get();
                let binary = binary_rules
                    .concat(
                        binary_manual
                            .semijoin_on(verts.clone(), |&((x, _), _)| x)
                            .semijoin_on(verts.clone(), |&((_, y), _)| y)
                            .map(|(xy, (i, _))| (xy, Path::Indirect(i))),
                    )
                    .split()
                    .named("binary");
                let closure_raw = transitive_closure(
                    &mut self.context.borrow_mut(),
                    verts.clone(),
                    binary.clone().map(fst).collect(),
                )
                .named("closure_raw");
                let (closure, closure_output) =
                    closure_raw.assert_1to1_with_output(&self.context.borrow());
                let closure = closure.split().named("closure");
                let self_implied = closure
                    .clone()
                    .flat_map(|((x, y), d)| {
                        if y == !x {
                            Some((d, (y.var(), y)))
                        } else {
                            None
                        }
                    })
                    .dynamic()
                    .named("self_implied");
                let rule_sizes = self.rule_sizes.get();
                let chosen_rule_index = rule_sizes
                    .clone()
                    .flat_map(|(i, rs)| if rs >= 3 { Some(i) } else { None })
                    .named("chosen_rule_index");
                let chosen_rule = active
                    .rule
                    .clone()
                    .semijoin(chosen_rule_index)
                    .split()
                    .named("chosen_rule");
                let pairings = chosen_rule
                    .clone()
                    .map(move |(i, x)| {
                        let mut h = DefaultHasher::new();
                        (i, x).hash(&mut h);
                        (i, (h.finish(), x))
                    })
                    .reduce(|_, xs: &BTreeMap<(u64, Assig), isize>| {
                        let mut iter = xs.iter().assert_ones();
                        let &(_, x) = iter.next().expect("Empty rule");
                        let &(_, y) = iter.next().expect("Unary rule");
                        let &(_, z) = iter.next().expect("Binary rule");
                        VecMap::new(vec![((x, y), 1), ((x, z), 1), ((y, z), 1)])
                    })
                    .split()
                    .named("pairings");
                let implied = closure.map(fst).split().named("implied");
                let both_implied = pairings
                    .clone()
                    .map(snd)
                    .triangles(implied.clone(), implied.clone())
                    .map(|(x, y, z)| ((x, y), z))
                    .named("both_implied");
                let candidates = pairings
                    .map(swap)
                    .join(both_implied)
                    .map(snd)
                    .distinct()
                    .named("candidates");
                let impl_count = chosen_rule
                    .clone()
                    .join(candidates)
                    .dynamic() // TODO Why does removing this cause the compiler to hang?
                    .semijoin_on_snd(implied.clone())
                    .map(|(i, (_, y))| (i, y))
                    .counts()
                    .map(|((i, y), c)| {
                        assert!(c >= 2);
                        (i, (y, c))
                    })
                    .named("impl_count");
                let critical = impl_count
                    .join(rule_sizes.clone())
                    .flat_map(|(i, ((y, c), rs))| if c >= rs - 1 { Some((i, y)) } else { None })
                    .split()
                    .named("critical");
                let new_binary = chosen_rule
                    .join(critical.clone())
                    .map(swap)
                    .antijoin(implied.clone())
                    .map(|((x, y), i)| ((!x, y), i))
                    .split()
                    .named("new_binary");
                let new_unary = critical
                    .map(swap)
                    .concat(new_binary.clone().map(|((_, y), i)| (y, i)).negate())
                    .named("new_unary");
                let context = self.context.borrow();
                Binary {
                    binary_input,
                    binary_output: binary
                        .map(|((x, y), i)| (x, (y, i)))
                        .dynamic()
                        .named("binary_output")
                        .get_arrangement(&context),
                    binary_by_level,
                    closure: Box::new(closure_output),
                    self_implied: self_implied.get_arrangement(&context),
                    other_un_impls: new_unary
                        .map(|(x, i)| (x.var(), (x, i)))
                        .dynamic()
                        .named("other_un_impls")
                        .get_arrangement(&context),
                    other_bin_impls: new_binary
                        .antijoin(implied)
                        .dynamic()
                        .named("other_bin_impls")
                        .get_arrangement(&context),
                }
            }
        }
        fn transitive_closure<T: Key>(
            context: &mut CreationContext,
            verts: Collection<T>,
            edges: Collection<(T, T)>,
        ) -> Collection<'static, ((T, T), usize)> {
            let mut subcontext = context.subgraph::<usize>();
            let (var, c) = subcontext.variable();
            let c = c.named("c");
            let nextdists = verts
                .map(|x| ((x.clone(), x), 0))
                .enter()
                .concat(
                    c.map(|(d, (x, y))| (y, (x, d)))
                        .join(edges.enter())
                        .map(|(_, ((x, d), y))| ((x, y), d + 1)),
                )
                .group_min()
                .split()
                .named("nextdists");
            var.set(nextdists.clone().map(swap));
            nextdists.leave(&subcontext.finish()).collect()
        }
    }
    pub mod collections {
        use super::Binary;
        use crate::primitives::{Assig, RuleIndex};
        use crate::program::RulesCollections;
        use dc2::{Collection, CreationContext};
        use lazy_fields::{self, with_lazy_fields, LazyField};
        use std::cell::RefCell;
        type PField<'a, T> = LazyField<'a, SolverCollections<'a>, T>;
        type RuleSizesCollection = Collection<'static, (RuleIndex, isize)>;
        pub struct SolverCollections<'a> {
            pub context: RefCell<CreationContext>,
            pub all_lits: PField<'a, Collection<'static, Assig>>,
            pub base: PField<'a, RulesCollections>,
            pub active_base: PField<'a, RulesCollections>,
            pub rem_lits: PField<'a, Collection<'static, Assig>>,
            pub active: PField<'a, RulesCollections>,
            pub rule_sizes: PField<'a, RuleSizesCollection>,
            pub binary: PField<'a, Binary>,
        }
        impl<'a> SolverCollections<'a> {
            pub fn new() -> Self {
                with_lazy_fields(
                    move |r: &mut lazy_fields::Register<'a, SolverCollections<'a>>| {
                        SolverCollections {
                            context: RefCell::new(CreationContext::new()),
                            base: r.field(SolverCollections::make_base),
                            all_lits: r.field(SolverCollections::make_all_lits),
                            active_base: r.field(SolverCollections::make_active_base),
                            rem_lits: r.field(SolverCollections::make_rem_lits),
                            active: r.field(SolverCollections::make_active),
                            rule_sizes: r.field(SolverCollections::make_rule_sizes),
                            binary: r.field(SolverCollections::make_binary),
                        }
                    },
                )
            }
            fn make_all_lits(&self) -> Collection<'static, Assig> {
                unimplemented!()
            }
            fn make_rem_lits(&self) -> Collection<'static, Assig> {
                unimplemented!()
            }
            fn make_base(&self) -> RulesCollections {
                unimplemented!()
            }
            fn make_active_base(&self) -> RulesCollections {
                unimplemented!()
            }
            fn make_active(&self) -> RulesCollections {
                unimplemented!()
            }
            fn make_rule_sizes(&self) -> RuleSizesCollection {
                unimplemented!()
            }
        }
    }
    mod learnt {
        use crate::ops::RcRaw;
        use crate::primitives::{Assig, RuleIndex};
        use std::cell::RefCell;
        use std::cmp::Ordering;
        use std::collections::HashMap;
        #[derive(Clone, PartialEq, Eq, Hash, Debug)]
        pub struct RefRule(RcRaw<RefCell<RuleBuilder>>);
        #[derive(Debug)]
        enum RuleBuilder {
            Leaf(RuleIndex),
            Node(RefRule, HashMap<Assig, RefRule>),
        }
        impl PartialOrd for RefRule {
            fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
                unimplemented!()
            }
        }
        impl Ord for RefRule {
            fn cmp(&self, _other: &Self) -> Ordering {
                unimplemented!()
            }
        }
    }
    mod primitives {
        use super::learnt::RefRule;
        use crate::primitives::{MicroLevel, RuleIndex};
        pub type DecisionLevel = usize;
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct AssignInfo {
            pub decision_level: DecisionLevel,
            pub cause: Cause,
            pub micro_level: MicroLevel,
        }
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub enum Cause {
            Decision,
            InferredFrom(RuleIndex),
            Pure,
            BinaryChains(RefRule),
        }
    }
    use self::binary::Binary;
    use self::collections::SolverCollections;
}
mod tuple {
    pub fn fst<A, B>((a, _): (A, B)) -> A {
        a
    }
    pub fn snd<A, B>((_, b): (A, B)) -> B {
        b
    }
    pub fn swap<A, B>((a, b): (A, B)) -> (B, A) {
        (b, a)
    }
}

use self::solver::collections::SolverCollections;

fn main() {
    SolverCollections::new();
}
