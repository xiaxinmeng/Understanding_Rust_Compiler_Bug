rust
Steal{
  value: RwLock(RefCell{
    value: Some(Body{
      basic_blocks: BasicBlocks{
        basic_blocks: [
          BasicBlockData{
            statements: [
              StorageLive(_3),
              StorageLive(_4),
              StorageLive(_5),
              StorageLive(_6),
              StorageLive(_7),
              StorageLive(_8),
              StorageLive(_9),
              _9=[
                const""
              ],
              _8=&_9,
              _7=&(*_8),
              _6=move_7as&[
                &str
              ](Pointer(Unsize)),
              StorageDead(_7),
              StorageLive(_10),
              StorageLive(_11),
              StorageLive(_12),
              StorageLive(_13),
              StorageLive(_14),
              StorageLive(_15),
              StorageLive(_16),
              _16=&(*(_1.0: &TileDb)),
              _15=&(*_16)
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 9: 13: 40(#9),
                scope: scope[
                  0
                ]
              },
              kind: _14=core: : fmt: : ArgumentV1: : <'_>: : new_debug: : <TileDb>(move_15)->[
                return: bb1,
                unwind: bb9
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              StorageDead(_15),
              _13=[
                move_14
              ],
              StorageDead(_14),
              _12=&_13,
              _11=&(*_12),
              _10=move_11as&[
                core: : fmt: : ArgumentV1<'_>
              ](Pointer(Unsize)),
              StorageDead(_11)
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
                scope: scope[
                  0
                ]
              },
              kind: _5=Arguments: : <'_>: : new_v1(move_6,
              move_10)->[
                return: bb2,
                unwind: bb9
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              StorageDead(_10),
              StorageDead(_6)
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 19: 120: 80(#7),
                scope: scope[
                  0
                ]
              },
              kind: _4=format(move_5)->[
                return: bb3,
                unwind: bb9
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              StorageDead(_5),
              FakeRead(ForLet(None),
              _4),
              StorageDead(_16),
              StorageDead(_13),
              StorageDead(_12),
              StorageDead(_9),
              StorageDead(_8),
              _3=move_4
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 122: 5: 122: 6(#7),
                scope: scope[
                  0
                ]
              },
              kind: drop(_4)->[
                return: bb4,
                unwind: bb9
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              StorageDead(_4)
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 9: 13: 40(#20),
                scope: scope[
                  0
                ]
              },
              kind: drop(((*_2).0: std: : string: : String))->[
                return: bb5,
                unwind: bb6
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              ((*_2).0: std: : string: : String)=move_3,
              _0=const()
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 39: 13: 40(#5),
                scope: scope[
                  0
                ]
              },
              kind: drop(_3)->[
                return: bb7,
                unwind: bb9
              ]
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [
              ((*_2).0: std: : string: : String)=move_3
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 9: 13: 40(#20),
                scope: scope[
                  0
                ]
              },
              kind: goto->bb8
            }),
            is_cleanup: true
          },
          BasicBlockData{
            statements: [
              StorageDead(_3)
            ],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 40: 13: 40(#5),
                scope: scope[
                  0
                ]
              },
              kind: return
            }),
            is_cleanup: false
          },
          BasicBlockData{
            statements: [],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 39: 13: 40(#5),
                scope: scope[
                  0
                ]
              },
              kind: drop(_3)->[
                return: bb9,
                unwindterminate
              ]
            }),
            is_cleanup: true
          },
          BasicBlockData{
            statements: [],
            terminator: Some(Terminator{
              source_info: SourceInfo{
                span: src/main.rs: 13: 9: 13: 40(#5),
                scope: scope[
                  0
                ]
              },
              kind: resume
            }),
            is_cleanup: true
          }
        ],
        cache: Cache{
          predecessors: OnceCell(Uninit),
          switch_sources: OnceCell(Uninit),
          is_cyclic: OnceCell(Uninit),
          postorder: OnceCell(Uninit)
        }
      },
      phase: Built,
      pass_count: 0,
      source: MirSource{
        instance: Item(WithOptConstParam{
          did: DefId(0: 20~mantle_diver[
            d58e
          ]: : {
            impl#0
          }: : inspect_mut: : {
            closure#1
          }),
          const_param_did: None
        }),
        promoted: None
      },
      source_scopes: [
        SourceScopeData{
          span: src/main.rs: 13: 9: 13: 40(#5),
          parent_scope: None,
          inlined: None,
          inlined_parent_scope: None,
          local_data: Set(SourceScopeLocalData{
            lint_root: HirId(DefId(0: 8~mantle_diver[
              d58e
            ]: : {
              impl#0
            }: : inspect_mut).21),
            safety: Safe
          })
        },
        SourceScopeData{
          span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 9: 122: 6(#7),
          parent_scope: Some(scope[
            0
          ]),
          inlined: None,
          inlined_parent_scope: None,
          local_data: Set(SourceScopeLocalData{
            lint_root: HirId(DefId(0: 8~mantle_diver[
              d58e
            ]: : {
              impl#0
            }: : inspect_mut).21),
            safety: Safe
          })
        }
      ],
      generator: None,
      local_decls: [
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: (),
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 9(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: [
            closure@src/main.rs: 13: 9: 13: 40
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(User(Var(VarBindingForm{
            binding_mode: BindByValue(Not),
            opt_ty_info: None,
            opt_match_place: Some((None,
            src/main.rs: 13: 9: 13: 40(#5))),
            pat_span: src/main.rs: 13: 9: 13: 40(#5)
          }))),
          internal: false,
          ty: &mutPlatformOutput,
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: std: : string: : String,
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 119: 23: 122: 6(#7),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(User(Var(VarBindingForm{
            binding_mode: BindByValue(Not),
            opt_ty_info: None,
            opt_match_place: Some((None,
            /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 19: 120: 80(#7))),
            pat_span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 13: 120: 16(#7)
          }))),
          internal: false,
          ty: std: : string: : String,
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 13: 120: 16(#7),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: std: : fmt: : Arguments<'_>,
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            &str
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            &str;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            &str;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(Boring),
          internal: false,
          ty: [
            &str;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            core: : fmt: : ArgumentV1<'_>
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            core: : fmt: : ArgumentV1<'_>;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(Boring),
          internal: false,
          ty: &[
            core: : fmt: : ArgumentV1<'_>;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(Boring),
          internal: false,
          ty: [
            core: : fmt: : ArgumentV1<'_>;1
          ],
          user_ty: None,
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: core: : fmt: : ArgumentV1<'_>,
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Mut,
          local_info: Set(Boring),
          internal: false,
          ty: &TileDb,
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#9),
            scope: scope[
              0
            ]
          }
        },
        LocalDecl{
          mutability: Not,
          local_info: Set(Boring),
          internal: false,
          ty: &TileDb,
          user_ty: None,
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#9),
            scope: scope[
              0
            ]
          }
        }
      ],
      user_type_annotations: [
        CanonicalUserTypeAnnotation{
          user_ty: Canonical{
            value: TypeOf(DefId(2: 9040~core[
              44ff
            ]: : fmt: : {
              impl#4
            }: : new_v1),
            UserSubsts{
              substs: [
                ReLateBound(DebruijnIndex(0),
                BoundRegion{
                  var: 0,
                  kind: BrAnon(None)
                })
              ],
              user_self_ty: Some(UserSelfTy{
                impl_def_id: DefId(2: 9037~core[
                  44ff
                ]: : fmt: : {
                  impl#4
                }),
                self_ty: std: : fmt: : Arguments<'_>
              })
            }),
            max_universe: U0,
            variables: [
              CanonicalVarInfo{
                kind: Region(U0)
              },
              CanonicalVarInfo{
                kind: Region(U0)
              }
            ]
          },
          span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 39: 120: 79(#9),
          inferred_ty: fn(&[
            &'staticstr
          ],
          &[
            core: : fmt: : ArgumentV1<'_>
          ])->std: : fmt: : Arguments<'_>{
            std: : fmt: : Arguments: : <'_>: : new_v1
          }
        },
        CanonicalUserTypeAnnotation{
          user_ty: Canonical{
            value: TypeOf(DefId(2: 41685~core[
              44ff
            ]: : fmt: : {
              impl#3
            }: : new_debug),
            UserSubsts{
              substs: [
                ReLateBound(DebruijnIndex(0),
                BoundRegion{
                  var: 0,
                  kind: BrAnon(None)
                }),
                ^1
              ],
              user_self_ty: Some(UserSelfTy{
                impl_def_id: DefId(2: 9030~core[
                  44ff
                ]: : fmt: : {
                  impl#3
                }),
                self_ty: core: : fmt: : ArgumentV1<'_>
              })
            }),
            max_universe: U0,
            variables: [
              CanonicalVarInfo{
                kind: Region(U0)
              },
              CanonicalVarInfo{
                kind: Ty(General(U0))
              },
              CanonicalVarInfo{
                kind: Region(U0)
              }
            ]
          },
          span: src/main.rs: 13: 9: 13: 40(#9),
          inferred_ty: for<'b>fn(&'bTileDb)->core: : fmt: : ArgumentV1<'b>{
            core: : fmt: : ArgumentV1: : <'_>: : new_debug: : <TileDb>
          }
        }
      ],
      arg_count: 2,
      spread_arg: None,
      var_debug_info: [
        VarDebugInfo{
          name: "o",
          source_info: SourceInfo{
            span: src/main.rs: 13: 9: 13: 40(#5),
            scope: scope[
              0
            ]
          },
          value: _2,
          argument_index: Some(2)
        },
        VarDebugInfo{
          name: "self__tile_db",
          source_info: SourceInfo{
            span: src/main.rs: 12: 25: 12: 29(#0),
            scope: scope[
              0
            ]
          },
          value: (*(_1.0: &TileDb)),
          argument_index: None
        },
        VarDebugInfo{
          name: "res",
          source_info: SourceInfo{
            span: /home/nilsh/projects/rust/library/alloc/src/macros.rs: 120: 13: 120: 16(#7),
            scope: scope[
              1
            ]
          },
          value: _4,
          argument_index: None
        }
      ],
      span: src/main.rs: 13: 9: 13: 40(#5),
      required_consts: [],
      is_polymorphic: false,
      injection_phase: None,
      tainted_by_errors: None
    })
  })
}
