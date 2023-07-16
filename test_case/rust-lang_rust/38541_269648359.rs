` Rust

        let expected = Node::new(ComparisonExpr).children(vec![
            Node::new(StringConcatExpr).children(vec![
                Node::new(RangeExpr).children(vec![
                    Node::new(AdditiveExpr).children(vec![
                        Node::new(MultiplicativeExpr).children(vec![
                            Node::new(UnionExpr).children(vec![
                                Node::new(IntersectExceptExpr).children(vec![
                                    Node::new(InstanceofExpr).children(vec![
                                        Node::new(TreatExpr).children(vec![
                                            Node::new(CastableExpr).children(vec![
                                                Node::new(CastExpr).children(vec![
                                                    Node::new(ArrowExpr).children(vec![
                                                        Node::new(UnaryExpr).children(vec![
                                                            Node::new(ValueExpr).children(vec![
                                                                Node::new(SimpleMapExpr).children(vec![
                                                                    Node::new(PathExpr).children(vec![
                                                                        Node::new(RelativePathExpr).children(vec![
                                                                            Node::new(StepExpr).children(vec![
                                                                                Node::new(PostfixExpr).children(vec![
                                                                                    Node::new(PrimaryExpr).children(vec![
                                                                                        Node::new(VarRef).children(vec![
                                                                                            Node::new(Operator).data("$"),
                                                                                            Node::new(VarName).data("CashFlowOperatingActivities"), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]),
            Node::new(Meta(Opt)).children(vec![
                Node::new(Meta(Seq)).children(vec![
                    Node::new(Whitespace).data(" "),
                    Node::new(Meta(Or)).children(vec![
                        Node::new(GeneralComp).children(vec![
                            Node::new(Operator).data("="), ]), ]),
                    Node::new(Whitespace).data(" \n"),
                    Node::new(StringConcatExpr).children(vec![
                        Node::new(RangeExpr).children(vec![
                            Node::new(AdditiveExpr).children(vec![
                                Node::new(MultiplicativeExpr).children(vec![
                                    Node::new(UnionExpr).children(vec![
                                        Node::new(IntersectExceptExpr).children(vec![
                                            Node::new(InstanceofExpr).children(vec![
                                                Node::new(TreatExpr).children(vec![
                                                    Node::new(CastableExpr).children(vec![
                                                        Node::new(CastExpr).children(vec![
                                                            Node::new(ArrowExpr).children(vec![
                                                                Node::new(UnaryExpr).children(vec![
                                                                    Node::new(Meta(MultStar)).children(vec![
                                                                        Node::new(Meta(Or)).children(vec![
                                                                            Node::new(Operator).data("-"), ]),
                                                                        Node::new(Meta(Opt)).children(vec![
                                                                            Node::new(Meta(Seq)).children(vec![
                                                                                Node::new(Whitespace).data(" "), ]), ]), ]),
                                                                    Node::new(ValueExpr).children(vec![
                                                                        Node::new(SimpleMapExpr).children(vec![
                                                                            Node::new(PathExpr).children(vec![
                                                                                Node::new(RelativePathExpr).children(vec![
                                                                                    Node::new(StepExpr).children(vec![
                                                                                        Node::new(PostfixExpr).children(vec![
                                                                                            Node::new(PrimaryExpr).children(vec![
                                                                                                Node::new(FunctionCall).children(vec![
                                                                                                    Node::new(EQName).children(vec![
                                                                                                        Node::new(QName).children(vec![
                                                                                                            Node::new(UnprefixedName).data("sum"), ]), ]),
                                                                                                    Node::new(ArgumentList).children(vec![
                                                                                                        Node::new(Delimiter).data("("),
                                                                                                        Node::new(Meta(Opt)).children(vec![
                                                                                                            Node::new(Meta(Seq)).children(vec![
                                                                                                                Node::new(Argument).children(vec![
                                                                                                                    Node::new(ExprSingle).children(vec![
                                                                                                                        Node::new(OrExpr).children(vec![
                                                                                                                            Node::new(AndExpr).children(vec![
                                                                                                                                Node::new(ComparisonExpr).children(vec![
                                                                                                                                    Node::new(StringConcatExpr).children(vec![
                                                                                                                                        Node::new(RangeExpr).children(vec![
                                                                                                                                            Node::new(AdditiveExpr).children(vec![
                                                                                                                                                Node::new(MultiplicativeExpr).children(vec![
                                                                                                                                                    Node::new(UnionExpr).children(vec![
                                                                                                                                                        Node::new(IntersectExceptExpr).children(vec![
                                                                                                                                                            Node::new(InstanceofExpr).children(vec![
                                                                                                                                                                Node::new(TreatExpr).children(vec![
                                                                                                                                                                    Node::new(CastableExpr).children(vec![
                                                                                                                                                                        Node::new(CastExpr).children(vec![
                                                                                                                                                                            Node::new(ArrowExpr).children(vec![
                                                                                                                                                                                Node::new(UnaryExpr).children(vec![
                                                                                                                                                                                    Node::new(ValueExpr).children(vec![
                                                                                                                                                                                        Node::new(SimpleMapExpr).children(vec![
                                                                                                                                                                                            Node::new(PathExpr).children(vec![
                                                                                                                                                                                                Node::new(RelativePathExpr).children(vec![
                                                                                                                                                                                                    Node::new(StepExpr).children(vec![
                                                                                                                                                                                                        Node::new(PostfixExpr).children(vec![
                                                                                                                                                                                                            Node::new(PrimaryExpr).children(vec![
                                                                                                                                                                                                                Node::new(VarRef).children(vec![
                                                                                                                                                                                                                    Node::new(Operator).data("$"),
                                                                                                                                                                                                                    Node::new(VarName).data("varArc_ConsolidatedCashFlowStatement_MsgConsolidatedSumOfChildrenParentDebit1_ChildrenOfCashFlowOperatingActivitiesCredit"), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]),
                                                                                                        Node::new(Delimiter).data(")"), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]),
                                Node::new(Meta(MultStar)).children(vec![
                                    Node::new(Meta(Opt)).children(vec![
                                        Node::new(Meta(Seq)).children(vec![
                                            Node::new(Whitespace).data(" \n"), ]), ]),
                                    Node::new(Meta(Or)).children(vec![
                                        Node::new(Operator).data("+"), ]),
                                    Node::new(Meta(Opt)).children(vec![
                                        Node::new(Meta(Seq)).children(vec![
                                            Node::new(Whitespace).data(" "), ]), ]),
                                    Node::new(MultiplicativeExpr).children(vec![
                                        Node::new(UnionExpr).children(vec![
                                            Node::new(IntersectExceptExpr).children(vec![
                                                Node::new(InstanceofExpr).children(vec![
                                                    Node::new(TreatExpr).children(vec![
                                                        Node::new(CastableExpr).children(vec![
                                                            Node::new(CastExpr).children(vec![
                                                                Node::new(ArrowExpr).children(vec![
                                                                    Node::new(UnaryExpr).children(vec![
                                                                        Node::new(ValueExpr).children(vec![
                                                                            Node::new(SimpleMapExpr).children(vec![
                                                                                Node::new(PathExpr).children(vec![
                                                                                    Node::new(RelativePathExpr).children(vec![
                                                                                        Node::new(StepExpr).children(vec![
                                                                                            Node::new(PostfixExpr).children(vec![
                                                                                                Node::new(PrimaryExpr).children(vec![
                                                                                                    Node::new(FunctionCall).children(vec![
                                                                                                        Node::new(EQName).children(vec![
                                                                                                            Node::new(QName).children(vec![
                                                                                                                Node::new(UnprefixedName).data("sum"), ]), ]),
                                                                                                        Node::new(ArgumentList).children(vec![
                                                                                                            Node::new(Delimiter).data("("),
                                                                                                            Node::new(Meta(Opt)).children(vec![
                                                                                                                Node::new(Meta(Seq)).children(vec![
                                                                                                                    Node::new(Argument).children(vec![
                                                                                                                        Node::new(ExprSingle).children(vec![
                                                                                                                            Node::new(OrExpr).children(vec![
                                                                                                                                Node::new(AndExpr).children(vec![
                                                                                                                                    Node::new(ComparisonExpr).children(vec![
                                                                                                                                        Node::new(StringConcatExpr).children(vec![
                                                                                                                                            Node::new(RangeExpr).children(vec![
                                                                                                                                                Node::new(AdditiveExpr).children(vec![
                                                                                                                                                    Node::new(MultiplicativeExpr).children(vec![
                                                                                                                                                        Node::new(UnionExpr).children(vec![
                                                                                                                                                            Node::new(IntersectExceptExpr).children(vec![
                                                                                                                                                                Node::new(InstanceofExpr).children(vec![
                                                                                                                                                                    Node::new(TreatExpr).children(vec![
                                                                                                                                                                        Node::new(CastableExpr).children(vec![
                                                                                                                                                                            Node::new(CastExpr).children(vec![
                                                                                                                                                                                Node::new(ArrowExpr).children(vec![
                                                                                                                                                                                    Node::new(UnaryExpr).children(vec![
                                                                                                                                                                                        Node::new(ValueExpr).children(vec![
                                                                                                                                                                                            Node::new(SimpleMapExpr).children(vec![
                                                                                            Node::new(PathExpr).children(vec![
                                                                                              Node::new(RelativePathExpr).children(vec![
                                                                                                Node::new(StepExpr).children(vec![
                                                                                                  Node::new(PostfixExpr).children(vec![
                                                                                                    Node::new(PrimaryExpr).children(vec![
                                                                                                      Node::new(VarRef).children(vec![
                                                                                                        Node::new(Operator).data("$"),
                                                                                                        Node::new(VarName).data("varArc_ConsolidatedCashFlowStatement_MsgConsolidatedSumOfChildrenParentDebit1_ChildrenOfCashFlowOperatingActivitiesDebit"), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]),
                                                  Node::new(Delimiter).data(")"), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]), ]);

